use crate::{makepad_draw::*, makepad_widgets::widget::*, makepad_widgets::*};

live_design! {
    FishConnectionWidget = {{FishConnectionWidget}} {}
}

#[derive(Clone, WidgetAction)]
pub enum FishConnectionWidgetAction {
    None,
    Clicked,
    Pressed,
    Released,
}

#[derive(Live)]
pub struct FishConnectionWidget {
    #[animator]
    animator: Animator,
    #[live]
    draw_line: DrawLine,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live(true)]
    grab_key_focus: bool,
    #[live]
    pub text: RcStringMut,
}

impl LiveHook for FishConnectionWidget {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, FishConnectionWidget)
    }
}

impl Widget for FishConnectionWidget {
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut WidgetScope,
    ) -> WidgetActions {
        let mut actions = WidgetActions::new();
        let uid = self.widget_uid();
        self.animator_handle_event(cx, event);
        match event.hits(cx, self.draw_line.area()) {
            Hit::FingerDown(_fe) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.draw_line.area());
                }
                actions.push_single(uid, &scope.path, ButtonAction::Pressed);
                self.animator_play(cx, id!(hover.pressed));
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    actions.push_single(uid, &scope.path, ButtonAction::Clicked);
                    if fe.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    actions.push_single(uid, &scope.path, ButtonAction::Released);
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
        actions
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_line.redraw(cx)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut WidgetScope, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk_fishconnection(cx, walk);
        WidgetDraw::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }

    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
}

impl FishConnectionWidget {
    pub fn draw_walk_fishconnection(&mut self, cx: &mut Cx2d, walk: Walk) {
        self.draw_line.begin(cx, walk, self.layout);
        self.draw_line.end(cx);
    }
}

#[derive(Clone, Debug, PartialEq, WidgetRef)]
pub struct FishConnectionWidgetRef(WidgetRef);
/*
impl FishConnectionWidgetRef {
    pub fn clicked(&self, actions: &WidgetActions) -> bool {
        if let Some(item) = actions.find_single_action(self.widget_uid()) {
            if let FishConnectionWidgetAction::Clicked = item.cast() {
                return true;
            }
        }
        false
    }

    pub fn pressed(&self, actions: &WidgetActions) -> bool {
        if let Some(item) = actions.find_single_action(self.widget_uid()) {
            if let FishConnectionWidgetAction::Pressed = item.cast() {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug, WidgetSet)]
pub struct FishConnectionWidgetSet(WidgetSet);
impl FishConnectionWidgetSet {
    pub fn clicked(&self, actions: &WidgetActions) -> bool {
        for FishConnectionWidget in self.iter() {
            if FishConnectionWidget.clicked(actions) {
                return true;
            }
        }
        false
    }
    pub fn pressed(&self, actions: &WidgetActions) -> bool {
        for FishConnectionWidget in self.iter() {
            if FishConnectionWidget.pressed(actions) {
                return true;
            }
        }
        false
    }
}*/
