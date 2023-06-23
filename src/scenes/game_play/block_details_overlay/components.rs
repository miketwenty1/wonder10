use bevy::prelude::*;

#[derive(Component)]
pub struct DetailsMenu;

#[derive(Component)]
pub struct BuyBdBlockButton;

#[derive(Component)]
pub struct BackBdButton;

#[derive(Component)]
pub struct BlockchainMenuNode;

#[derive(Component, Debug)]
pub enum EditableButton {
    LN,
    Color,
    Msg,
}

#[derive(Component)]
pub enum EditableText {
    LN,
    Color,
    Msg,
}

#[derive(Component)]
pub struct LightningAddressText;

#[derive(Component)]
pub struct LightningAddressButton(pub bool);

#[derive(Component)]
pub struct ColorText;
#[derive(Component)]
pub struct ColorButton(pub bool);

#[derive(Component)]
pub struct MessageText;
#[derive(Component)]
pub struct MessageButton(pub bool);
