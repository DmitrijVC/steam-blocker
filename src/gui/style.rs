use iced::{button, Background, Color, Vector, container, scrollable};
use iced::widget::button::Style;


pub struct Container;

pub struct ContainerStdout;
pub struct ScrollableStdout;

/// ToDo: Implement other button types
pub enum Button {
    Primary
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: Color::from_rgb8(42, 42, 43).into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }
}

impl container::StyleSheet for ContainerStdout {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Color::from_rgb8(230, 225, 225).into(),
            background: Color::from_rgb8(15, 15, 15).into(),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgb8(10, 10, 10).into(),
        }
    }
}

impl scrollable::StyleSheet for ScrollableStdout {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Color::from_rgb8(22, 22, 22).into(),
            border_radius: 0.0,
            border_width: 2.0,
            border_color: Color::from_rgb8(15, 15, 15).into(),
            scroller: scrollable::Scroller {
                color: Color::from_rgb8(222, 222, 222).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            }
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Color::from_rgb8(22, 22, 22).into(),
            border_radius: 0.0,
            border_width: 2.0,
            border_color: Color::from_rgb8(15, 15, 15).into(),
            scroller: scrollable::Scroller {
                color: Color::from_rgb8(181, 181, 181).into(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            }
        }
    }
}

/// ToDo: Redesign button style
impl button::StyleSheet for Button {
    fn active(&self) -> Style {
        button::Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
            border_radius: 12.0,
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Color::from_rgb(69.0, 72.0, 77.0)
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}
