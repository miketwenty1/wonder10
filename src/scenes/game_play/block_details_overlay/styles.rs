use bevy::prelude::*;
pub const BACKGROUND_COLOR: Color = Color::rgba(0.05, 0.05, 0.05, 0.995);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const INPUT_COLOR: Color = Color::rgb(0.1, 0.6, 0.01);
pub const BG_INPUT_COLOR: Color = Color::rgb(0.85, 0.85, 0.85);
pub const BG_BUY_BTN_COLOR: Color = Color::rgb(0.05, 0.25, 0.05);
pub const BG_BUY_BTN_HOVER_COLOR: Color = Color::rgb(0.1, 0.35, 0.1);

pub fn get_title_text_style(asset_server: &Res<AssetServer>, size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: size,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_button_text_style(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: 32.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_bd_menu_style() -> Style {
    Style {
        //size: Size::width(Val::Percent(100.0)),
        //flex_direction: FlexDirection::Row,
        align_content: AlignContent::Center,
        position_type: PositionType::Absolute, // Needed to display separately from HUD.
        display: Display::Flex,                // Hidden by Default
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Style::DEFAULT
    }
}

pub fn get_bd_menu_container_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        //gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
        ..Style::DEFAULT
    }
}

pub fn get_button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub fn get_editable_button_style() -> Style {
    Style {
        //size: Size::new(Val::Px(200.0), Val::Px(80.0)),
        //justify_content: JustifyContent::Center,
        // align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}
