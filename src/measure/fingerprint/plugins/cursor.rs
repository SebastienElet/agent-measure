use super::{PluginSelection, manifest as plugin_manifest, safe_label, within};
use crate::measure::MeasureError;
use crate::measure::fingerprint::SelectedRoot;
use std::path::{Component, Path, PathBuf};

pub fn selected(home: &Path) -> Result<PluginSelection, MeasureError> {
    let mut selection = PluginSelection {
        roots: Vec::new(),
        limitations: vec![
            "cursor marketplace plugin activation is not observable; marketplace caches excluded"
                .to_owned(),
        ],
        markers: Vec::new(),
    };
    let path = std::env::var_os("AGENT_MEASURE_CURSOR_PLUGINS").map_or_else(
        || home.join(".config/agent-measure/cursor-plugins.json"),
        PathBuf::from,
    );
    if !path.is_absolute() {
        return Err(MeasureError::new(
            "AGENT_MEASURE_CURSOR_PLUGINS must be absolute",
        ));
    }
    let manifest_file = plugin_manifest::read(&path)?;
    if let Some(marker) = manifest_file.marker {
        selection.markers.push(format!("cursor:manifest:{marker}"));
        selection
            .limitations
            .push("oversized plugin manifest uses size and boundary windows".to_owned());
    }
    let Some(contents) = manifest_file.contents else {
        selection.limitations.push(
            "cursor local plugin declarations are not observable; local plugin files excluded"
                .to_owned(),
        );
        return Ok(selection);
    };
    let ids: Vec<String> = serde_json::from_str(&contents)
        .map_err(|error| MeasureError::new(format!("cursor plugin selection: {error}")))?;
    let local = home.join(".cursor/plugins/local");
    for id in ids {
        let path = Path::new(&id);
        if id.is_empty()
            || path.components().count() != 1
            || !matches!(path.components().next(), Some(Component::Normal(_)))
            || id.contains(['/', '\\'])
        {
            return Err(MeasureError::new(
                "cursor plugin ID must be one path component",
            ));
        }
        add_local(&mut selection, &local, &id);
    }
    Ok(selection)
}

fn add_local(selection: &mut PluginSelection, local: &Path, id: &str) {
    let path = local.join(id);
    if within(&path, local) {
        selection.roots.push(SelectedRoot {
            label: Path::new("home/.cursor/plugins/active").join(safe_label(id)),
            path,
            bounded: true,
        });
    } else {
        selection
            .markers
            .push(format!("cursor:{id}:local-root-escape"));
        selection.limitations.push(
            "cursor declared local plugin root is unresolved; plugin files excluded".to_owned(),
        );
    }
}
