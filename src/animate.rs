use bevy::prelude::*;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn name(&self) -> &str {
        "Stateful Animate Play Plugin"
    }

    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, animate_sprite);

        #[cfg(feature = "debug")]
        app.register_type::<AnimationNode>()
            .register_type::<AnimationTimer>();
    }
}

#[derive(Component, Reflect)]
pub struct AnimationNode {
    index: usize,
    first: usize,
    last: usize,
    repeat: bool,
}

impl AnimationNode {
    pub fn new(first: usize, last: usize, repeat: bool) -> Self {
        Self {
            index: first,
            first,
            last,
            repeat,
        }
    }

    pub fn set_repeate(&mut self, turn: bool) {
        self.repeat = turn;
    }

    pub fn set_indices(&mut self, (first, last): (usize, usize)) {
        self.first = first;
        self.last = last;
    }

    pub fn next(&mut self) -> Option<usize> {
        if self.index > self.last {
            None
        } else if self.index <= self.first {
            self.index = self.first + 1;
            Some(self.first)
        } else {
            let result = Some(self.index);
            self.index += 1;
            result
        }
    }

    pub fn reset(&mut self) {
        self.index = self.first;
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct AnimationTimer(pub Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationNode,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut node, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.finished() {
            if let Some(index) = node.next() {
                sprite.index = index;
                timer.reset();
                timer.unpause();
            } else if node.repeat {
                node.reset();
            }
        }
    }
}
