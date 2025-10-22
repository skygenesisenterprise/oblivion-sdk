# Themes

Themes in Oblivion UI allow you to customize the appearance of your applications. They provide a consistent way to define colors, fonts, and other visual properties.

## Theme Structure

Themes are defined by the `Theme` struct:

```rust
pub struct Theme {
    pub primary_color: (u8, u8, u8),
    pub secondary_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
    pub text_color: (u8, u8, u8),
    pub font_size: u32,
}
```

## Default Theme

```rust
impl Default for Theme {
    fn default() -> Self {
        Theme {
            primary_color: (0, 122, 255),     // Blue
            secondary_color: (142, 142, 147),  // Gray
            background_color: (255, 255, 255), // White
            text_color: (0, 0, 0),             // Black
            font_size: 14,
        }
    }
}
```

## Using Themes

Pass themes to the rendering engine:

```rust
let theme = Theme::default();
let mut engine = SDLEngine::new("My App", 800, 600)?;
engine.run(root_component, &theme)?;
```

## Component Theming

Components receive the theme in their `render` method:

```rust
impl Component for MyComponent {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Use theme colors
        renderer.set_draw_color(theme.primary_color);
        renderer.draw_rect(self.x, self.y, self.width, self.height);

        // Use theme font size
        renderer.set_font_size(theme.font_size);
        renderer.draw_text(&self.text, self.x, self.y);
    }
}
```

## Custom Themes

Create custom themes for different contexts:

```rust
// Dark theme
let dark_theme = Theme {
    primary_color: (255, 149, 0),      // Orange
    secondary_color: (142, 142, 147),  // Gray
    background_color: (28, 28, 30),    // Dark gray
    text_color: (255, 255, 255),       // White
    font_size: 14,
};

// High contrast theme
let high_contrast_theme = Theme {
    primary_color: (255, 255, 255),    // White
    secondary_color: (255, 255, 255),  // White
    background_color: (0, 0, 0),       // Black
    text_color: (255, 255, 255),       // White
    font_size: 16,
};
```

## Theme Variants

### Light Theme
```rust
let light_theme = Theme {
    primary_color: (0, 122, 255),
    secondary_color: (142, 142, 147),
    background_color: (255, 255, 255),
    text_color: (0, 0, 0),
    font_size: 14,
};
```

### Dark Theme
```rust
let dark_theme = Theme {
    primary_color: (10, 132, 255),
    secondary_color: (142, 142, 147),
    background_color: (0, 0, 0),
    text_color: (255, 255, 255),
    font_size: 14,
};
```

### Colorblind-friendly Theme
```rust
let colorblind_theme = Theme {
    primary_color: (51, 51, 51),       // Dark gray
    secondary_color: (128, 128, 128),  // Medium gray
    background_color: (255, 255, 255), // White
    text_color: (0, 0, 0),             // Black
    font_size: 16,                     // Larger for accessibility
};
```

## Dynamic Theming

Change themes at runtime:

```rust
struct ThemedApp {
    current_theme: State<Theme>,
    content: Box<dyn Component>,
}

impl ThemedApp {
    fn toggle_theme(&self) {
        let new_theme = if self.is_dark_theme() {
            Theme::default()  // Light
        } else {
            create_dark_theme()
        };
        self.current_theme.set(new_theme);
    }

    fn is_dark_theme(&self) -> bool {
        self.current_theme.get().background_color == (0, 0, 0)
    }
}
```

## Component-specific Styling

Override theme properties for specific components:

```rust
struct StyledButton {
    base_theme: Theme,
    custom_color: Option<(u8, u8, u8)>,
}

impl Component for StyledButton {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Merge theme with custom properties
        let effective_theme = Theme {
            primary_color: self.custom_color.unwrap_or(theme.primary_color),
            ..*theme
        };

        // Render with effective theme
        // ...
    }
}
```

## Theme Inheritance

Create theme hierarchies:

```rust
struct ThemeFamily {
    base: Theme,
    variants: HashMap<String, Theme>,
}

impl ThemeFamily {
    fn get_variant(&self, name: &str) -> &Theme {
        self.variants.get(name).unwrap_or(&self.base)
    }
}

// Usage
let themes = ThemeFamily {
    base: Theme::default(),
    variants: hashmap! {
        "error" => Theme {
            primary_color: (255, 59, 48),  // Red
            ..Theme::default()
        },
        "success" => Theme {
            primary_color: (52, 199, 89),  // Green
            ..Theme::default()
        },
    },
};
```

## Platform-specific Themes

Adapt themes for different platforms:

```rust
#[cfg(target_os = "macos")]
fn platform_theme() -> Theme {
    Theme {
        font_size: 13,  // macOS standard
        ..Theme::default()
    }
}

#[cfg(target_os = "windows")]
fn platform_theme() -> Theme {
    Theme {
        font_size: 12,  // Windows standard
        ..Theme::default()
    }
}

#[cfg(target_os = "linux")]
fn platform_theme() -> Theme {
    Theme {
        font_size: 11,  // Linux standard
        ..Theme::default()
    }
}
```

## Accessibility Considerations

### High Contrast
```rust
let high_contrast_theme = Theme {
    primary_color: (255, 255, 255),
    secondary_color: (255, 255, 255),
    background_color: (0, 0, 0),
    text_color: (255, 255, 255),
    font_size: 18,  // Larger text
};
```

### Large Text
```rust
let large_text_theme = Theme {
    font_size: 24,
    ..Theme::default()
};
```

### Color Blind Friendly
```rust
let cb_friendly_theme = Theme {
    primary_color: (0, 0, 139),        // Dark blue
    secondary_color: (139, 69, 19),    // Saddle brown
    background_color: (255, 255, 255),
    text_color: (0, 0, 0),
    font_size: 14,
};
```

## Theme Persistence

Save and load themes:

```rust
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SavedTheme {
    primary_color: [u8; 3],
    secondary_color: [u8; 3],
    background_color: [u8; 3],
    text_color: [u8; 3],
    font_size: u32,
}

impl From<Theme> for SavedTheme {
    fn from(theme: Theme) -> Self {
        SavedTheme {
            primary_color: [theme.primary_color.0, theme.primary_color.1, theme.primary_color.2],
            secondary_color: [theme.secondary_color.0, theme.secondary_color.1, theme.secondary_color.2],
            background_color: [theme.background_color.0, theme.background_color.1, theme.background_color.2],
            text_color: [theme.text_color.0, theme.text_color.1, theme.text_color.2],
            font_size: theme.font_size,
        }
    }
}

fn save_theme(theme: &Theme, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let saved = SavedTheme::from(*theme);
    let json = serde_json::to_string(&saved)?;
    fs::write(path, json)?;
    Ok(())
}

fn load_theme(path: &str) -> Result<Theme, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(path)?;
    let saved: SavedTheme = serde_json::from_str(&json)?;
    Ok(Theme {
        primary_color: (saved.primary_color[0], saved.primary_color[1], saved.primary_color[2]),
        secondary_color: (saved.secondary_color[0], saved.secondary_color[1], saved.secondary_color[2]),
        background_color: (saved.background_color[0], saved.background_color[1], saved.background_color[2]),
        text_color: (saved.text_color[0], saved.text_color[1], saved.text_color[2]),
        font_size: saved.font_size,
    })
}
```

## Best Practices

1. **Use semantic color names** - primary, secondary, etc.
2. **Provide theme variants** - light, dark, high contrast
3. **Test themes on target devices** - different screen sizes/DPIs
4. **Consider accessibility** - contrast ratios, font sizes
5. **Allow user customization** - theme picker in settings
6. **Version themes** - handle theme format changes

## Common Patterns

### Theme Provider Component
```rust
struct ThemeProvider {
    theme: State<Theme>,
    child: Box<dyn Component>,
}

impl Component for ThemeProvider {
    fn render(&self, renderer: &mut dyn Renderer, _theme: &Theme) {
        // Override theme for children
        self.child.render(renderer, &self.theme.get());
    }

    fn handle_event(&mut self, event: &Event) {
        self.child.handle_event(event);
    }
}
```

### Conditional Styling
```rust
impl Component for StatusIndicator {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        let color = match self.status {
            Status::Success => (52, 199, 89),   // Green
            Status::Warning => (255, 149, 0),   // Orange
            Status::Error => (255, 59, 48),     // Red
        };

        renderer.set_draw_color(color);
        renderer.draw_circle(self.x, self.y, self.radius);
    }
}
```

## Future Enhancements

- [ ] CSS-like styling system
- [ ] Theme animations/transitions
- [ ] Dynamic theme generation
- [ ] Theme marketplace/integration
- [ ] Advanced typography settings
- [ ] Icon theming
- [ ] Spacing and layout theming