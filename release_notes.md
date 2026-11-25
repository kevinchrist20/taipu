# Taipu v0.3.0

This release introduces **Visual Progress Analytics**, featuring an organic, Japanese Bonsai-inspired **Growth Tree**, interactive **Speed & Accuracy Trend Charts**, and **Daily Practice Streaks**.

## Highlights

- **Bonsai Growth Tree**: A procedural vector growth tree that sprouts, branches, grows lush canopy foliage, and bursts into floral bloom across 5 distinct evolutionary stages as your typing skill and consistency advance.
- **Dynamic Foliage & Floral Blooms**: Canopy density scales with your typing accuracy precision, while vibrant blossoms emerge dynamically based on active consecutive practice streaks.
- **Speed & Accuracy Trend Chart**: Zero-dependency interactive time-series chart with 7-day, 30-day, and all-time range views, dual WPM gradient curve and accuracy trajectory line, and mouse inspection crosshair with live session tooltips.
- **Daily Practice Streaks & 28-Day Punchcard**: Track consecutive active days with an animated flame badge, best streak milestones, and a visual 4-week activity punchcard showing daily completions on hover.
- **Refreshed Stats Experience**: Redesigned Performance & Mastery dashboard organizing growth visuals, streak telemetry, key metrics, curriculum progress, and activity logs into a cohesive desktop layout.

## Technical Updates

- Added Rust stats models (`DailyActivityStat`, `StreakInfo`, `TreeStageInfo`) with synchronized TypeScript bindings via `ts-rs`.
- Implemented SQLite time-bucketed daily aggregation and calendar-date streak calculation in `stats_service.rs`.
- Created lightweight, responsive SVG components (`GrowthTree.vue`, `StatsTrendChart.vue`, `StreakCard.vue`) fully styled with Taipu's design tokens and light/dark theme support.
- Added interactive milestone preview controls allowing typists to explore all 5 tree evolution tiers.
