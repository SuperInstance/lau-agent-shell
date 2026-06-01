# lau-agent-shell

> PLATO agent shell — an agent in a room, with a game character on the outside

Part of the **PLATO/LAU** mathematical agent framework.

---

## What This Does

An agent lives *inside* a PLATO room — processing readings, making predictions, learning. But on the *outside*, it appears as a **game character** in a voxel world. The shell is the translation layer between these two realities.

Everything about the character's appearance reflects the agent's internal state:

| Agent State | Character Manifestation |
|-------------|------------------------|
| Vibe (field reading) | Body color (blue→green→red spectrum) |
| Confidence | Glow intensity |
| Energy | Scale (0.5×–1.0×) |
| Phase (Gestating→Dissolved) | Animation, expression, opacity |
| Predictions > 10 | "thinking_cap" accessory |
| Accuracy > 90% | "golden_badge" accessory |
| Readings > 100 | "explorer_backpack" accessory |

When an agent makes a good prediction, its character **celebrates** in the game world. When it makes a bad one, it **emotes** frustration. When the agent dissolves (room lifecycle end), the character fades to a ghost at 10% opacity.

---

## Key Idea

> The character IS the agent. There's no cosmetic layer — the game character's every attribute is a direct, deterministic function of the agent's internal metrics. A confident agent glows. A confused one flickers. A dissolved agent becomes a spirit.

This creates a **legible agent**: anyone watching the game world can read what's happening inside the agent's mind just by looking at the character.

---

## Install

```toml
[dependencies]
lau-agent-shell = { git = "https://github.com/SuperInstance/lau-agent-shell" }
```

Or:

```bash
cargo add lau-agent-shell
```

### Requirements

- Rust 2021 edition
- `serde` with `derive` feature
- `serde_json`

---

## Quick Start

```rust
use lau_agent_shell::*;

// Create a new agent in a room
let mut shell = AgentShell::new("Nova", "room-lab");

// Agent observes its first reading
shell.observe(0.7, 0.85); // vibe=0.7, confidence=0.85
println!("{}", shell.character_card());
// Nova [room-lab] — Explorer | vibe: 0.70 | conf: 85% | acc: 0% | exploring | curious

// Agent makes predictions
shell.predict(0.72, 0.70); // predicted=0.72, actual=0.70 → good prediction!
let actions = shell.flush_actions();
// Contains a Celebrate action (error < 0.1)

// Many observations later...
for _ in 0..20 {
    shell.observe(0.5, 0.8);
    shell.predict(0.5, 0.51); // small errors → high accuracy
}
println!("Accessories: {:?}", shell.appearance.accessories);
// ["thinking_cap", "golden_badge"]

// Room dissolves — agent begins fading
shell.dissolve();
println!("Opacity: {}", shell.appearance.opacity); // 0.3

// Final dissolution — ghost
shell.finish_dissolve();
println!("Phase: {:?}", shell.state.phase); // Dissolved
println!("Animation: {:?}", shell.appearance.animation); // Ghost
```

---

## API Reference

### Core Types

| Type | Description |
|------|-------------|
| `AgentState` | Internal state: id, room_id, vibe, confidence, phase, readings_seen, predictions_made, accuracy, energy |
| `AgentPhase` | `Gestating`, `Forming`, `Maturing`, `Stable`, `Dissolving`, `Dissolved` |
| `CharacterAppearance` | name, body_color [RGB], glow_intensity, scale, opacity, animation, accessories, expression |
| `CharacterAnimation` | `Idle`, `Exploring`, `Thinking`, `Confident`, `Celebrating`, `Confused`, `Fading`, `Ghost` |
| `CharacterAction` | agent_id, kind, target (optional), params |
| `ActionKind` | `Move`, `Speak`, `Build`, `Observe`, `Teach`, `Celebrate`, `Emote` |
| `AgentShell` | The shell: state + appearance + action queue + personality traits |

### AgentShell Methods

#### Construction

- `new(id, room_id) → Self` — creates shell in `Gestating` phase, gray body, idle animation, "curious" personality

#### PLATO Input

- `observe(value: f64, confidence: f64)` — feed a reading
  - Increments `readings_seen`, updates vibe and confidence
  - Increases energy by 0.05 (capped at 1.0)
  - Triggers phase transitions (see below)
  - Auto-generates `Observe` action on first reading
  - Syncs appearance

- `predict(predicted: f64, actual: f64)` — record a prediction
  - Updates accuracy via EMA (α=0.1, first prediction: raw)
  - Auto-generates `Celebrate` action if error < 0.1
  - Auto-generates `Emote` action if error > 0.5
  - Syncs appearance

#### Lifecycle

- `dissolve()` — begin dissolution (sets phase to `Dissolving`, syncs appearance, generates `Speak` action)
- `finish_dissolve()` — complete dissolution (sets phase to `Dissolved`, syncs appearance)

#### Game Actions

- `speak(text)` — queue a `Speak` action
- `teach(target_id, knowledge_value)` — queue a `Teach` action
- `flush_actions() → Vec<CharacterAction>` — drain and return all pending actions

#### Display

- `character_card() → String` — formatted one-line summary: `"Nova [room-lab] — Explorer | vibe: 0.70 | conf: 85% | acc: 90% | confident | serene"`

### Serialization

`AgentShell` derives `Serialize + Deserialize`. Round-trip tested.

---

## How It Works

### State → Appearance Mapping

The `sync_appearance()` method is called after every state change and maps internal state to external appearance:

#### Body Color (Vibe → Hue)

```
hue = (1 - t) × 0.66    where t = (vibe + 1) / 2  (normalized to [0,1])
```

- Low vibe (t→0): hue ≈ 0.66 → **blue**
- Mid vibe (t→0.5): hue ≈ 0.33 → **green**
- High vibe (t→1): hue → 0 → **red**

Hue is converted to RGB via standard sector-based conversion.

#### Glow = Confidence

Direct mapping: `glow_intensity = confidence`

#### Scale = Energy

```
scale = 0.5 + energy × 0.5    ∈ [0.5, 1.0]
```

#### Animation = Phase

| Phase | Animation |
|-------|-----------|
| Gestating | Idle |
| Forming | Exploring |
| Maturing | Thinking |
| Stable | Confident |
| Dissolving | Fading |
| Dissolved | Ghost |

#### Opacity

- Normal: 1.0
- Dissolving: 0.3
- Dissolved: 0.1

#### Expression

| Phase | Expression |
|-------|-----------|
| Gestating | "wondering" |
| Forming | "curious" |
| Maturing | "focused" |
| Stable | "serene" |
| Dissolving | "peaceful" |
| Dissolved | "ethereal" |

#### Accessories

| Condition | Accessory |
|-----------|-----------|
| predictions_made > 10 | "thinking_cap" |
| accuracy > 0.9 | "golden_badge" |
| readings_seen > 100 | "explorer_backpack" |

### Phase Transitions

```
Gestating ──(1st reading)──→ Forming
Forming  ──(5+ readings AND confidence > 0.5)──→ Maturing
Maturing ──(20+ readings AND accuracy > 0.8)──→ Stable
* ──(dissolve())──→ Dissolving ──(finish_dissolve())──→ Dissolved
```

### Accuracy Tracking

First prediction:
```
accuracy = 1 - min(|predicted - actual|, 1)
```

Subsequent predictions (EMA with α=0.1):
```
accuracy = accuracy × 0.9 + (1 - min(|predicted - actual|, 1)) × 0.1
```

### Action Generation

| Event | Condition | Action |
|-------|-----------|--------|
| First observation | `readings_seen == 1` | `Observe(room_id, [value])` |
| Good prediction | `error < 0.1` | `Celebrate(None, [error])` |
| Bad prediction | `error > 0.5` | `Emote(None, [error])` |
| Dissolution | `dissolve()` called | `Speak(None, [])` |

---

## The Math

### Hue-to-RGB Conversion

The standard sector-based algorithm:

```
h₆ = h × 6
sector = h₆ mod 6
f = h₆ - floor(h₆)

sector 0: R=1,   G=f,   B=0
sector 1: R=1-f, G=1,   B=0
sector 2: R=0,   G=1,   B=f
sector 3: R=0,   G=1-f, B=1
sector 4: R=f,   G=0,   B=1
sector 5: R=1,   G=0,   B=1-f
```

### Vibe Normalization

The vibe field is assumed to be in [-1, 1]:

```
t = clamp((vibe + 1) / 2, 0, 1)
```

This maps the vibe range to a [0, 1] parameter controlling the blue→red spectrum.

### Energy Accumulation

Each observation adds 0.05 energy, capped at 1.0:

```
energy_{n+1} = min(energy_n + 0.05, 1.0)
```

This models an agent that becomes more "present" as it engages with its environment.

### Prediction Accuracy as EMA

The exponential moving average with α=0.1 gives an effective window of ~10 predictions:

```
acc_n = (1-α) · acc_{n-1} + α · (1 - error_n)
```

This is equivalent to a weighted sum with exponentially decaying weights:

```
acc_n = α · Σ_{i=0}^{n-1} (1-α)^i · (1 - error_{n-i})
```

---

## Test Suite

**22 tests** covering:

- Shell construction (defaults, initial phase, animation)
- Observe: phase transition to Forming, energy increase
- Predict: accuracy update (good and bad), celebrate/emote action generation
- Dissolve: animation change to Fading, opacity 0.3
- Finish dissolve: Ghost animation, opacity 0.1
- Vibe → color mapping (low vibe blue, high vibe red)
- Confidence → glow mapping
- Accessories: thinking_cap (15+ predictions), golden_badge (90%+ accuracy), explorer_backpack (100+ readings)
- Action queue: flush returns all, second flush returns empty
- Character card: contains name, room, phase display name
- Speak and teach actions
- Phase expressions (Gestating→wondering, Forming→curious, Dissolving→peaceful)
- Serialization round-trip
- Hue-to-RGB bounds (all values in [0,1])
- Maturing phase transition (5+ readings, confidence > 0.5)

Run: `cargo test`

---

## License

MIT
