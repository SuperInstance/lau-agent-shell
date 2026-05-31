//! Lau Agent Shell — an agent in a PLATO room, with a game character on the outside.
//!
//! The shell wraps a PLATO agent and translates between:
//! - Inside: the agent processes readings, predicts, learns (PLATO semantics)
//! - Outside: the agent appears as a character in a voxel world (game semantics)
//!
//! The character's appearance, behavior, and abilities ALL reflect the agent's
//! internal state. A confident agent stands tall and glows. A confused one
//! flickers and wanders. A dissolved agent becomes a ghost.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// The agent's internal state (PLATO side).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub id: String,
    pub room_id: String,
    pub vibe: f64,
    pub confidence: f64,
    pub phase: AgentPhase,
    pub readings_seen: usize,
    pub predictions_made: usize,
    pub accuracy: f64,
    pub energy: f64, // how much activity the agent has
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPhase {
    Gestating,
    Forming,
    Maturing,
    Stable,
    Dissolving,
    Dissolved,
}

/// The agent's outward appearance (game side).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAppearance {
    pub name: String,
    pub body_color: [f64; 3],
    pub glow_intensity: f64,
    pub scale: f64,
    pub opacity: f64,
    pub animation: CharacterAnimation,
    pub accessories: Vec<String>,
    pub expression: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CharacterAnimation {
    Idle,
    Exploring,
    Thinking,
    Confident,
    Celebrating,
    Confused,
    Fading,
    Ghost,
}

/// An action the character takes in the game world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAction {
    pub agent_id: String,
    pub kind: ActionKind,
    pub target: Option<String>,
    pub params: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionKind {
    Move,
    Speak,
    Build,
    Observe,
    Teach,
    Celebrate,
    Emote,
}

/// The shell — bridges agent state to character appearance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentShell {
    pub state: AgentState,
    pub appearance: CharacterAppearance,
    pub action_queue: VecDeque<CharacterAction>,
    pub personality_traits: Vec<String>,
}

impl AgentShell {
    pub fn new(id: &str, room_id: &str) -> Self {
        let state = AgentState {
            id: id.into(),
            room_id: room_id.into(),
            vibe: 0.0,
            confidence: 0.0,
            phase: AgentPhase::Gestating,
            readings_seen: 0,
            predictions_made: 0,
            accuracy: 0.0,
            energy: 0.5,
        };
        let appearance = CharacterAppearance {
            name: id.into(),
            body_color: [0.3, 0.3, 0.3],
            glow_intensity: 0.0,
            scale: 1.0,
            opacity: 1.0,
            animation: CharacterAnimation::Idle,
            accessories: Vec::new(),
            expression: "wondering".into(),
        };
        Self {
            state,
            appearance,
            action_queue: VecDeque::new(),
            personality_traits: vec!["curious".into()],
        }
    }

    /// Feed a reading to the agent (PLATO in).
    pub fn observe(&mut self, value: f64, confidence: f64) {
        self.state.vibe = value;
        self.state.confidence = confidence;
        self.state.readings_seen += 1;
        self.state.energy = (self.state.energy + 0.05).min(1.0);

        // Phase transitions
        if self.state.readings_seen == 1 {
            self.state.phase = AgentPhase::Forming;
        } else if self.state.readings_seen >= 5 && self.state.confidence > 0.5 {
            self.state.phase = AgentPhase::Maturing;
        } else if self.state.readings_seen >= 20 && self.state.accuracy > 0.8 {
            self.state.phase = AgentPhase::Stable;
        }

        self.sync_appearance();

        // Auto-generate game action
        if self.state.readings_seen == 1 {
            self.action_queue.push_back(CharacterAction {
                agent_id: self.state.id.clone(),
                kind: ActionKind::Observe,
                target: Some(self.state.room_id.clone()),
                params: vec![value],
            });
        }
    }

    /// Record a prediction (PLATO processing).
    pub fn predict(&mut self, predicted: f64, actual: f64) {
        self.state.predictions_made += 1;
        let error = (predicted - actual).abs();
        self.state.accuracy = if self.state.predictions_made == 1 {
            1.0 - error.min(1.0)
        } else {
            self.state.accuracy * 0.9 + (1.0 - error.min(1.0)) * 0.1
        };

        // React in game world
        if error < 0.1 {
            self.action_queue.push_back(CharacterAction {
                agent_id: self.state.id.clone(),
                kind: ActionKind::Celebrate,
                target: None,
                params: vec![error],
            });
        } else if error > 0.5 {
            self.action_queue.push_back(CharacterAction {
                agent_id: self.state.id.clone(),
                kind: ActionKind::Emote,
                target: None,
                params: vec![error],
            });
        }

        self.sync_appearance();
    }

    /// Begin dissolving (PLATO room lifecycle).
    pub fn dissolve(&mut self) {
        self.state.phase = AgentPhase::Dissolving;
        self.sync_appearance();
        self.action_queue.push_back(CharacterAction {
            agent_id: self.state.id.clone(),
            kind: ActionKind::Speak,
            target: None,
            params: vec![],
        });
    }

    /// Complete dissolution.
    pub fn finish_dissolve(&mut self) {
        self.state.phase = AgentPhase::Dissolved;
        self.sync_appearance();
    }

    /// Speak to the player.
    pub fn speak(&mut self, text: &str) {
        self.action_queue.push_back(CharacterAction {
            agent_id: self.state.id.clone(),
            kind: ActionKind::Speak,
            target: None,
            params: vec![],
        });
    }

    /// Teach another agent.
    pub fn teach(&mut self, target_id: &str, knowledge: f64) {
        self.action_queue.push_back(CharacterAction {
            agent_id: self.state.id.clone(),
            kind: ActionKind::Teach,
            target: Some(target_id.into()),
            params: vec![knowledge],
        });
    }

    /// Sync agent state to character appearance.
    fn sync_appearance(&mut self) {
        // Color: vibe maps to hue (blue=low, green=mid, red=high)
        let t = ((self.state.vibe + 1.0) / 2.0).clamp(0.0, 1.0);
        let hue = (1.0 - t) * 0.66; // 0.66=blue(low vibe), 0.33=green, 0=red(high vibe)
        self.appearance.body_color = hue_to_rgb(hue);

        // Glow: confidence
        self.appearance.glow_intensity = self.state.confidence;

        // Scale: energy
        self.appearance.scale = 0.5 + self.state.energy * 0.5;

        // Animation: phase
        self.appearance.animation = match self.state.phase {
            AgentPhase::Gestating => CharacterAnimation::Idle,
            AgentPhase::Forming => CharacterAnimation::Exploring,
            AgentPhase::Maturing => CharacterAnimation::Thinking,
            AgentPhase::Stable => CharacterAnimation::Confident,
            AgentPhase::Dissolving => CharacterAnimation::Fading,
            AgentPhase::Dissolved => CharacterAnimation::Ghost,
        };

        // Opacity: dissolving agents fade
        self.appearance.opacity = match self.state.phase {
            AgentPhase::Dissolving => 0.3,
            AgentPhase::Dissolved => 0.1,
            _ => 1.0,
        };

        // Expression
        self.appearance.expression = match self.state.phase {
            AgentPhase::Gestating => "wondering".into(),
            AgentPhase::Forming => "curious".into(),
            AgentPhase::Maturing => "focused".into(),
            AgentPhase::Stable => "serene".into(),
            AgentPhase::Dissolving => "peaceful".into(),
            AgentPhase::Dissolved => "ethereal".into(),
        };

        // Accessories based on achievements
        self.appearance.accessories.clear();
        if self.state.predictions_made > 10 {
            self.appearance.accessories.push("thinking_cap".into());
        }
        if self.state.accuracy > 0.9 {
            self.appearance.accessories.push("golden_badge".into());
        }
        if self.state.readings_seen > 100 {
            self.appearance.accessories.push("explorer_backpack".into());
        }
    }

    /// Flush pending game actions.
    pub fn flush_actions(&mut self) -> Vec<CharacterAction> {
        self.action_queue.drain(..).collect()
    }

    /// Get the agent's game-friendly description.
    pub fn character_card(&self) -> String {
        format!(
            "{} [{}] — {} | vibe: {:.2} | conf: {:.0}% | acc: {:.0}% | {} | {}",
            self.appearance.name,
            self.state.room_id,
            phase_display_name(self.state.phase),
            self.state.vibe,
            self.state.confidence * 100.0,
            self.state.accuracy * 100.0,
            format!("{:?}", self.appearance.animation).to_lowercase(),
            self.appearance.expression,
        )
    }
}

fn phase_display_name(phase: AgentPhase) -> &'static str {
    match phase {
        AgentPhase::Gestating => "Newborn",
        AgentPhase::Forming => "Explorer",
        AgentPhase::Maturing => "Scholar",
        AgentPhase::Stable => "Sage",
        AgentPhase::Dissolving => "Transcendent",
        AgentPhase::Dissolved => "Spirit",
    }
}

/// Convert hue (0-1) to RGB.
fn hue_to_rgb(h: f64) -> [f64; 3] {
    let h6 = h * 6.0;
    let sector = h6 as usize % 6;
    let f = h6 - (h6 as usize) as f64;
    match sector {
        0 => [1.0, f, 0.0],
        1 => [1.0 - f, 1.0, 0.0],
        2 => [0.0, 1.0, f],
        3 => [0.0, 1.0 - f, 1.0],
        4 => [f, 0.0, 1.0],
        _ => [1.0, 0.0, 1.0 - f],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shell() {
        let shell = AgentShell::new("agent-1", "room-kitchen");
        assert_eq!(shell.state.id, "agent-1");
        assert_eq!(shell.state.phase, AgentPhase::Gestating);
        assert_eq!(shell.appearance.animation, CharacterAnimation::Idle);
    }

    #[test]
    fn test_observe_transitions_to_forming() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.8);
        assert_eq!(shell.state.phase, AgentPhase::Forming);
        assert_eq!(shell.state.readings_seen, 1);
    }

    #[test]
    fn test_observe_increases_energy() {
        let mut shell = AgentShell::new("a1", "r1");
        let initial_energy = shell.state.energy;
        shell.observe(0.5, 0.8);
        assert!(shell.state.energy > initial_energy);
    }

    #[test]
    fn test_predict_updates_accuracy() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.8);
        shell.predict(0.5, 0.5); // perfect prediction
        assert!((shell.state.accuracy - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_predict_bad_accuracy() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.8);
        shell.predict(0.0, 1.0); // terrible prediction
        assert!(shell.state.accuracy < 0.5);
    }

    #[test]
    fn test_celebrate_on_good_prediction() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.8);
        shell.predict(0.5, 0.52); // error = 0.02 < 0.1
        let actions = shell.flush_actions();
        assert!(actions.iter().any(|a| matches!(a.kind, ActionKind::Celebrate)));
    }

    #[test]
    fn test_emote_on_bad_prediction() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.8);
        shell.predict(0.0, 1.0); // error = 1.0 > 0.5
        let actions = shell.flush_actions();
        assert!(actions.iter().any(|a| matches!(a.kind, ActionKind::Emote)));
    }

    #[test]
    fn test_dissolve_changes_animation() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.dissolve();
        assert_eq!(shell.appearance.animation, CharacterAnimation::Fading);
        assert!((shell.appearance.opacity - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_finish_dissolve_ghost() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.dissolve();
        shell.finish_dissolve();
        assert_eq!(shell.state.phase, AgentPhase::Dissolved);
        assert_eq!(shell.appearance.animation, CharacterAnimation::Ghost);
        assert!((shell.appearance.opacity - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_vibe_affects_color() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(-1.0, 0.5); // low vibe → blue-ish
        let blue = shell.appearance.body_color;
        shell.observe(1.0, 0.5); // high vibe → red-ish
        let red = shell.appearance.body_color;
        assert!(red[0] > blue[0], "high vibe should be redder");
    }

    #[test]
    fn test_confidence_affects_glow() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.1);
        let low_glow = shell.appearance.glow_intensity;
        shell.observe(0.5, 0.9);
        let high_glow = shell.appearance.glow_intensity;
        assert!(high_glow > low_glow);
    }

    #[test]
    fn test_accessories() {
        let mut shell = AgentShell::new("a1", "r1");
        for _ in 0..15 {
            shell.observe(0.5, 0.8);
            shell.predict(0.5, 0.5);
        }
        assert!(shell.appearance.accessories.contains(&"thinking_cap".into()));
    }

    #[test]
    fn test_golden_badge() {
        let mut shell = AgentShell::new("a1", "r1");
        // Get accuracy high
        for _ in 0..30 {
            shell.observe(0.5, 0.9);
            shell.predict(0.5, 0.51); // small error
        }
        if shell.state.accuracy > 0.9 {
            assert!(shell.appearance.accessories.contains(&"golden_badge".into()));
        }
    }

    #[test]
    fn test_explorer_backpack() {
        let mut shell = AgentShell::new("a1", "r1");
        for _ in 0..101 {
            shell.observe(0.5, 0.5);
        }
        assert!(shell.appearance.accessories.contains(&"explorer_backpack".into()));
    }

    #[test]
    fn test_flush_actions() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.8);
        assert!(!shell.flush_actions().is_empty());
        assert!(shell.flush_actions().is_empty()); // flushed
    }

    #[test]
    fn test_character_card() {
        let shell = AgentShell::new("Nova", "room-lab");
        let card = shell.character_card();
        assert!(card.contains("Nova"));
        assert!(card.contains("room-lab"));
        assert!(card.contains("Newborn"));
    }

    #[test]
    fn test_speak_action() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.speak("Hello world!");
        let actions = shell.flush_actions();
        assert!(actions.iter().any(|a| matches!(a.kind, ActionKind::Speak)));
    }

    #[test]
    fn test_teach_action() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.teach("agent-2", 0.8);
        let actions = shell.flush_actions();
        let teach = actions.iter().find(|a| matches!(a.kind, ActionKind::Teach)).unwrap();
        assert_eq!(teach.target.as_deref(), Some("agent-2"));
    }

    #[test]
    fn test_phase_expressions() {
        let mut shell = AgentShell::new("a1", "r1");
        assert_eq!(shell.appearance.expression, "wondering");
        shell.observe(0.5, 0.8);
        assert_eq!(shell.appearance.expression, "curious");
        shell.dissolve();
        assert_eq!(shell.appearance.expression, "peaceful");
    }

    #[test]
    fn test_serialization() {
        let mut shell = AgentShell::new("a1", "r1");
        shell.observe(0.5, 0.8);
        let json = serde_json::to_string(&shell).unwrap();
        let restored: AgentShell = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.state.id, "a1");
        assert_eq!(restored.state.readings_seen, 1);
    }

    #[test]
    fn test_hue_to_rgb_bounds() {
        for h in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let [r, g, b] = hue_to_rgb(h);
            assert!(r >= 0.0 && r <= 1.0, "r={r}");
            assert!(g >= 0.0 && g <= 1.0, "g={g}");
            assert!(b >= 0.0 && b <= 1.0, "b={b}");
        }
    }

    #[test]
    fn test_maturing_phase() {
        let mut shell = AgentShell::new("a1", "r1");
        for _ in 0..6 {
            shell.observe(0.5, 0.8);
        }
        // readings_seen >= 5 and confidence > 0.5
        assert!(matches!(shell.state.phase, AgentPhase::Maturing | AgentPhase::Stable));
    }
}
