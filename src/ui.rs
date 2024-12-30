use bevy::{prelude::*, text::FontSmoothing};

use self::{prompt::PromptUiPlugin, score::ScoreUiPlugin};
use crate::{score::Score, state::GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app //
      .init_resource::<GameFont>()
      .add_plugins((PromptUiPlugin, ScoreUiPlugin));
  }
}

#[derive(Resource, Deref)]
struct GameFont(TextFont);

impl FromWorld for GameFont {
  fn from_world(world: &mut World) -> Self {
    let font = world
      .resource::<AssetServer>()
      .load("fonts/FlappyBirdRegular.ttf");
    Self(TextFont {
      font,
      font_size: 28.0,
      font_smoothing: FontSmoothing::None,
    })
  }
}

mod score {
  use super::*;

  pub struct ScoreUiPlugin;

  impl Plugin for ScoreUiPlugin {
    fn build(&self, app: &mut App) {
      app //
        .add_systems(Startup, init_score_text)
        .add_systems(
          Update,
          update_score_text.run_if(in_state(GameState::Going)),
        );
    }
  }

  #[derive(Component)]
  struct ScoreText;

  fn init_score_text(mut commands: Commands, font: Res<GameFont>) {
    let font = font.clone();
    commands
      .spawn((
        Text::new("Score "), //
        font.clone(),
        Node {
          margin: UiRect::left(Val::Px(5.0)),
          ..default()
        },
      ))
      .with_child((
        TextSpan::new(format!("{}", *Score::default())),
        font,
        ScoreText,
      ));
  }

  fn update_score_text(
    score: Res<Score>,
    mut text_span: Single<&mut TextSpan, With<ScoreText>>,
  ) {
    if score.is_changed() {
      text_span.0 = format!("{}", **score);
    }
  }
}

mod prompt {
  use super::*;

  pub struct PromptUiPlugin;

  impl Plugin for PromptUiPlugin {
    fn build(&self, app: &mut App) {
      app
        .insert_resource(PromptAnimationTimer::from_seconds(0.7))
        .add_systems(
          OnEnter(GameState::Idle),
          (
            (init_prompt, show_prompt).chain(),
            reset_prompt_animation_timer,
          ),
        )
        .add_systems(
          OnEnter(GameState::GameOver),
          (show_prompt, reset_prompt_animation_timer),
        )
        .add_systems(
          OnEnter(GameState::Paused),
          (show_prompt, reset_prompt_animation_timer),
        )
        .add_systems(OnEnter(GameState::Going), hide_prompt)
        .add_systems(
          Update,
          (tick_prompt_animation_timer, animate_prompt).run_if(
            in_state(GameState::Idle)
              .or(in_state(GameState::GameOver))
              .or(in_state(GameState::Paused)),
          ),
        );
    }
  }

  #[derive(Component)]
  struct TextPrompt;

  fn init_prompt(mut commands: Commands, font: Res<GameFont>) {
    commands.spawn((
      TextPrompt,
      Text::default(),
      TextLayout::new_with_justify(JustifyText::Center),
      font.clone(),
      Node {
        width: Val::Percent(100.),
        top: Val::Percent(75.),
        align_content: AlignContent::Center,
        justify_content: JustifyContent::FlexEnd,
        ..default()
      },
    ));
  }

  fn show_prompt(
    state: Res<State<GameState>>,
    query: Single<(&mut Text, &mut Visibility), With<TextPrompt>>,
  ) {
    let (mut text, mut visibility) = query.into_inner();
    let prompt = match state.get() {
      GameState::Idle => "SPACE to fly\nESC to pause",
      GameState::Paused => "SPACE to continue",
      GameState::GameOver => "SPACE to restart",
      GameState::Going => {
        unreachable!("prompt shouldn't be displayed in this state")
      }
    };
    text.0 = prompt.into();
    *visibility = Visibility::Inherited;
  }

  #[derive(Resource, Deref, DerefMut)]
  struct PromptAnimationTimer(Timer);

  impl PromptAnimationTimer {
    pub fn from_seconds(seconds: f32) -> Self {
      Self(Timer::from_seconds(seconds, TimerMode::Repeating))
    }
  }

  fn tick_prompt_animation_timer(
    time: Res<Time>,
    mut timer: ResMut<PromptAnimationTimer>,
  ) {
    timer.tick(time.delta());
  }

  fn reset_prompt_animation_timer(mut timer: ResMut<PromptAnimationTimer>) {
    timer.reset();
  }

  fn animate_prompt(
    timer: Res<PromptAnimationTimer>,
    mut visibility: Single<
      &mut Visibility,
      Or<(With<TextPrompt>, With<TextPrompt>)>,
    >,
  ) {
    if timer.finished() {
      visibility.toggle_inherited_hidden();
    }
  }

  fn hide_prompt(mut visibility: Single<&mut Visibility, With<TextPrompt>>) {
    **visibility = Visibility::Hidden;
  }
}
