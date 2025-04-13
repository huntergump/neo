# Milestone 3 – Future Additions

## Chunk System Enhancements
- [x] Implement ECS events for chunk loading/unloading
- [x] Track all tile entities by TileId for lookup/despawn
- [x] Integrate visibility checks using Chunk.is_visible
- [ ] Move terrain generation to background tasks (threaded)
- [ ] Add support for dirty chunks (modified externally)

## Terrain System Enhancements
- [x] Biome color representation for rendering
- [x] Tile creation from coordinates
- [x] Dynamic biome threshold adjustment
- [x] Elevation bounds tracking
- [x] Basic unit tests for terrain generation
- [ ] Implement moisture and temperature generation
- [ ] Add terrain features (rivers, lakes, etc.)
- [ ] Improve biome transitions

## Code Refactoring Suggestions
- [ ] Rename `TerrainSystem` → `TerrainLayer` or `TerrainData`
  - Current name is confusing as it's not an ECS System in the Bevy sense
  - Renaming would prevent confusion with actual systems
- [ ] Consider storing chunk-local elevation stats per chunk (in `Chunk`)
  - Helps with local terrain rendering/analysis later
  - Defer to Milestone 3+ when rendering becomes more important
- [ ] If tile memory becomes a concern, store `Tile` structs in `Vec` with (q, r) index mapping
  - This optimization is likely unnecessary for now
  - Consider only if memory usage becomes a bottleneck

## Performance Optimizations
- [ ] Implement chunk LOD (Level of Detail) system
- [ ] Add chunk culling based on visibility
- [ ] Optimize chunk loading/unloading with priority queue
- [ ] Cache frequently accessed chunks
- [ ] Parallel terrain generation
  - Use Bevy's task system for chunk generation
  - Implement work stealing for better load balancing
  - Add progress tracking for long operations
- [ ] Terrain caching
  - Cache frequently accessed terrain data
  - Add terrain compression for storage

## Agent Integration
- [ ] Add agent pathfinding through chunks
- [ ] Implement agent memory of visited chunks
- [ ] Add chunk-based agent behavior triggers
- [ ] Support agent interactions with chunk features
- [ ] Terrain influence on agent behavior
  - Pathfinding based on terrain type
  - Resource availability by biome
  - Terrain-based shelter and hiding
- [ ] Dynamic terrain modification
  - Agent ability to modify terrain
  - Terrain regeneration over time
  - Environmental impact tracking

## Tick System Enhancements
- [ ] Implement tick rate control and throttling
  - Add configurable tick rates per agent type
  - Implement tick rate limiting based on system load
  - Add tick rate debugging tools
- [ ] Enhance tick event system
  - Add more granular tick events (pre-tick, post-tick)
  - Implement tick event batching for performance
  - Add tick event filtering and prioritization
- [ ] Optimize tick processing
  - Implement parallel tick processing for independent agents
  - Add tick dependency tracking between agents
  - Cache frequently accessed tick data
- [ ] Add tick debugging features
  - Tick visualization tools
  - Tick performance metrics
  - Tick event logging and replay
- [ ] Improve tick message system
  - Add typed message system for agent communication
  - Implement message queuing and prioritization
  - Add message validation and error handling
- [ ] Tick system monitoring
  - Add tick system health metrics
  - Implement tick system diagnostics
  - Create tick system performance dashboard

## Resource System Enhancements
- [ ] Implement resource flow networks
  - Create ResourceFlow or ResourceNetwork struct for managing dynamic flows
  - Support directional resource transfer between connected entities
  - Add flow rate controls and bottlenecks
- [ ] Add capacity tiering by upgrade level
  - Support for agent or building evolution
  - Implement upgrade paths for resource storage and processing
  - Add visual indicators for upgrade levels
- [ ] Develop UI integration for resources
  - Create resource overlay system for map/debug panels
  - Add resource indicators for entities with resources
  - Implement resource tooltips and detailed views
- [ ] Enable LLM-editable resource parameters
  - Allow natural language commands to modify thresholds and decay rates
  - Support commands like "Make food spoil faster" or "Increase energy regeneration"
  - Add validation and safety checks for parameter changes
- [ ] Implement historical metrics tracking
  - Track resource usage over time for analysis
  - Create visualization tools for resource trends
  - Support queries like "Graph Energy use over time"
- [ ] Add resource market system
  - Implement buy/sell mechanics for resources
  - Add price fluctuation based on supply and demand
  - Create market UI for resource trading
- [ ] Enhance resource events and notifications
  - Add more detailed resource change events
  - Implement notification system for important resource events
  - Create event filtering and prioritization

## Debug & Development
- [ ] Add chunk debug visualization
- [ ] Implement chunk statistics tracking
- [ ] Add chunk modification history
- [ ] Create chunk editor tools
- [ ] Terrain visualization tools
  - Height map visualization
  - Biome distribution view
  - Terrain statistics panel
- [ ] Terrain editing tools
  - In-game terrain editor
  - Terrain brush system
  - Terrain import/export

## Documentation
- [ ] Document chunk system architecture
- [ ] Add examples for chunk modification
- [ ] Create performance guidelines
- [ ] Document best practices for chunk usage
- [ ] Terrain generation documentation
- [ ] Biome system documentation
- [ ] Performance optimization guide
- [ ] Terrain modification API

## Weather System Enhancements
- [ ] Implement biome-specific weather patterns
  - Create weather profiles for each biome type
  - Add biome influence on temperature and precipitation
  - Implement weather transitions between biomes
- [ ] Add chunk-local weather systems
  - Create weather zones for different chunks
  - Implement weather propagation between chunks
  - Add weather boundary effects
- [ ] Develop weather forecasting system
  - Create prediction models for weather changes
  - Add weather trend analysis
  - Implement forecast visualization
- [ ] Integrate weather with agent behavior
  - Add weather influence on agent movement
  - Implement weather-based shelter seeking
  - Create weather-dependent resource gathering
- [ ] Add LLM control for weather
  - Enable natural language weather commands
  - Implement weather parameter adjustment
  - Add weather scenario creation
- [ ] Enhance weather visualization
  - Create weather overlay system
  - Add particle effects for precipitation
  - Implement dynamic lighting based on weather
- [ ] Implement seasonal weather cycles
  - Add seasonal temperature variations
  - Create seasonal precipitation patterns
  - Implement seasonal transition effects
- [ ] Add weather impact on resources
  - Create weather-dependent resource regeneration
  - Implement weather effects on resource quality
  - Add weather-based resource availability
- [ ] Add atmospheric pressure simulation
  - Track pressure changes for weather fronts
  - Implement pressure-based weather patterns
  - Add pressure influence on wind and precipitation
- [ ] Implement region-based weather
  - Add region_id or ChunkCoord to WeatherChanged events
  - Create weather zones with distinct characteristics
  - Implement weather propagation between regions
- [ ] Add wind-based effects
  - Influence agent movement based on wind
  - Implement wind-based fog or visibility effects
  - Add wind effects for flying entities
- [ ] Create weather forecast buffer
  - Store predicted weather changes
  - Allow agents to plan based on forecasted weather
  - Implement LLM-generated weather forecasts
- [ ] Add weather memory system
  - Store last N weather changes for trend analysis
  - Implement weather prediction based on historical data
  - Create weather pattern recognition for agent behavior
- [ ] Enhance weather diagnostics
  - Add detailed logging of temperature changes
  - Implement weather visualization tools
  - Create weather statistics dashboard

## Memory Management
- [ ] Implement comprehensive memory profiling
  - Add memory usage tracking for key resources
  - Create memory usage visualization tools
  - Set up memory usage alerts
- [ ] Optimize event handling
  - Implement event batching for better performance
  - Add event prioritization to reduce memory usage
  - Create event filtering system
- [ ] Add resource cleanup systems
  - Implement automatic cleanup for unused resources
  - Add resource pooling for frequently created/destroyed entities
  - Create resource lifecycle management
- [ ] Optimize entity management
  - Implement entity culling for off-screen entities
  - Add entity pooling for frequently spawned/despawned entities
  - Create entity lifecycle tracking
- [ ] Implement memory leak detection
  - Add memory leak detection tools
  - Create memory usage regression tests
  - Implement automatic memory leak reporting
- [ ] Add performance monitoring
  - Create performance profiling tools
  - Implement performance regression testing
  - Add performance optimization guidelines 