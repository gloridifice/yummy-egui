<div align="center">

# yummy_egui

[English](../README.md) | [简体中文](zh-CN.md) | [日本語](ja-JP.md) | Français

</div>

Une bibliothèque de widgets, formulaires de paramètres, listes de sélection et thèmes TOML pour egui 0.34.3, indépendante du moteur de rendu.

L’application gère le rendu, les polices, les fenêtres, la sélection et la persistance. `eframe` sert uniquement aux exemples.

## Démarrage rapide

Appliquez le thème une fois à l’initialisation, puis dessinez les widgets dans votre interface :

```rust
use yummy_egui::{ParameterList, ParameterSlider, Theme};

fn setup(ctx: &egui::Context) {
    Theme::default().apply(ctx);
}

fn settings_ui(ui: &mut egui::Ui, gain: &mut f32) {
    ParameterList::new("settings").show(ui, |rows| {
        rows.row("gain", "Gain", "Output level", |ui| {
            ui.add(ParameterSlider::new(gain, 0.0..=1.0).max_decimals(2));
        });
    });
}
```

L’application hôte doit fournir les polices : `Theme::apply` ne les installe ni ne les remplace. Ne réappliquez pas le thème à chaque image si vous souhaitez conserver les modifications faites dans l’éditeur de style.

Pour lancer les exemples autonomes, exécutez ces commandes à la racine du dépôt :

```sh
cargo run --example demo
cargo run --example style_editor
```

## Fonctionnalités

- Curseurs de paramètres pour entiers et flottants, avec échelles logarithmiques, pas, suffixes et animation au survol.
- Formulaires à deux colonnes avec libellés multilignes, infobulles, titres de groupes et identifiants de lignes explicites.
- Listes de sélection défilantes avec lignes désactivées et sélection gérée par l’application.
- Boutons de panneau, rangées d’actions, choix exclusifs, contrôles segmentés et titres H1–H3.
- Thèmes TOML intégrés, chargement explicite de thèmes externes et styles propres à chaque Context.
- Éditeur de style en direct pour les widgets personnalisés et les réglages courants d’egui ; modifications en mémoire uniquement, hors styles dock.
- Intégration facultative d’`egui_dock` 0.19 via la feature `dock`.

## Thèmes

Partez des fichiers de [`src/theme/default/`](../src/theme/default/) :

| Fichier | Réglages |
| --- | --- |
| `egui.toml` | Style de base d’egui |
| `yummy_egui.toml` | Widgets, listes et titres personnalisés |
| `egui_dock.toml` | Style dock, uniquement avec la feature `dock` |

Chargez un répertoire existant, puis appliquez explicitement le thème :

```rust
fn load_theme(ctx: &egui::Context) -> Result<(), yummy_egui::ThemeError> {
    let theme = yummy_egui::Theme::load_dir("themes/my-theme")?;
    theme.apply(ctx);
    Ok(())
}
```

Les fichiers absents utilisent les valeurs intégrées correspondantes. Chaque fichier fourni remplace un document entier, sans fusion champ par champ : copiez un fichier intégré avant de le modifier. Un fichier illisible ou invalide produit une erreur. Le chargement ne surveille pas les fichiers et n’enregistre pas les modifications. Pour charger des chaînes TOML, utilisez `Theme::from_sources(ThemeSources { .. })`.

Renommez les fichiers de thème externes `charming_egui.toml` en `yummy_egui.toml` ; l’ancien nom n’est plus chargé. Le schéma TOML et les noms de l’API Rust publique restent inchangés.

Lors de la migration d’un fichier externe `egui.toml` depuis egui 0.33, ajoutez `warn_if_rect_changes_id = false` et `show_focused_widget = false` dans sa section `[debug]`, si elle existe.

Pour les styles dock, activez `features = ["dock"]` sur la dépendance et passez `theme.dock_style().clone()` à `DockArea::style(...)`. `Theme::apply` ne modifie pas automatiquement le style des zones dock.

## Compilation depuis les sources

Avec Rust 1.92+ installé, lancez ces commandes à la racine du dépôt :

```sh
cargo build --release
cargo check --examples --all-features
cargo test --all-features
cargo doc --no-deps --all-features --open
```

Les exemples nécessitent un environnement graphique et les dépendances système du moteur Glow d’`eframe`.

## Licence

Ce projet est distribué sous [licence MIT](../LICENSE). Le thème egui adapté conserve sa [notice MIT de wgpu_pbr](../src/theme/default/LICENSE-wgpu_pbr.txt).

## Développement

- [Index de la documentation (en anglais)](../doco/README.md)
- [Architecture (en anglais)](../doco/architecture.md) et [guide de développement (en anglais)](../doco/development.md)
- [Publication sur crates.io (en anglais)](../doco/publishing.md)
- [API publique](../src/lib.rs)
- [Galerie de composants](../src/demo.rs) et [exemple autonome](../examples/demo.rs)
- [Exemple de l’éditeur de style](../examples/style_editor.rs)
- [Chargement des thèmes](../src/theme/mod.rs) et [définitions des styles personnalisés](../src/theme/style.rs)
