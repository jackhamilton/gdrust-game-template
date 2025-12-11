use opencompose_rs::configs::Button::ButtonConfig;
use opencompose_rs::configs::view_subtypes::view_alignment::Alignment;
use opencompose_rs::layout_compositor::Compositor;
use godot::classes::control::LayoutPreset;
use opencompose_rs::configs::View::ViewConfig;
use opencompose_rs::view_builder;

use godot::classes::IControl;
use godot::classes::Control;
use godot::{prelude::*};

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

#[derive(GodotClass, Debug)]
#[class(tool, base=Control)]
pub struct ButtonSampleUI {
    pub base: Base<Control>
}

#[godot_api]
impl IControl for ButtonSampleUI {
    fn init(base: Base<Control>) -> Self {
        Self {
            base
        }
    }

    fn enter_tree(&mut self) {
        let mut dsl_ast = view_builder! {
            Column {
                Text("Title")
                    .horizontal_text_alignment(alignment: Alignment::Center)
                    .font_size(24)
                Text("Subtitle")
                    .font_size(12)
                Row {
                    Button(action: || {
                        godot_print!("Pressed button 1");
                    }) {
                        Text("Action 1")
                            .alignment(horizontal: Alignment::Center, vertical: Alignment::Center)
                    }
                    .frame(width: 200, height: 30)
                    Button(action: || {
                        godot_print!("Pressed button 2");
                    }) {
                        Text("Action 2")
                            .alignment(horizontal: Alignment::Center, vertical: Alignment::Center)
                    }
                    .frame(width: 200, height: 30)
                }
            }
        };
        Compositor::layout_ast(&mut dsl_ast);
        let parser = ASTParser {
            ast: dsl_ast
        };
        let control_children = parser.parse();
        let mut base_mut = self.base_mut();
        base_mut.set_anchors_preset(LayoutPreset::CENTER);
        base_mut.add_child(&control_children);
        base_mut.set_custom_minimum_size(control_children.get_minimum_size());
    }
}

