use super::*;

pub fn abyss(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    println!("Abyss builder");
    let mut chain = BuilderChain::new(new_depth, width, height, "Abyss");
    chain.start_with(AbyssMapBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::LEFT, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(VoronoiSpawning::new());
    chain
}

pub struct AbyssMapBuilder {}

impl InitialMapBuilder for AbyssMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.empty_map(build_data);
        self.spawn_hell(build_data);
    }
}

impl AbyssMapBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn empty_map(&mut self, build_data: &mut BuilderMap) {
        build_data
            .map
            .tiles
            .iter_mut()
            .for_each(|t| *t = TileType::Floor);
    }

    fn spawn_hell(&mut self, build_data: &mut BuilderMap) {
        let (x, y) = (build_data.width / 2, build_data.height / 2);

        let altar = build_data.map.xy_idx(x, y);
        build_data.spawn_list.push((altar, "Altar".to_string()));
        let amulet = build_data.map.xy_idx(x, y);
        build_data
            .spawn_list
            .push((amulet, "Amulet of Yala".to_string()));
        let chasmfiend_spawn = build_data.map.xy_idx(x + 10, y + 10);
        build_data
            .spawn_list
            .push((chasmfiend_spawn, "Chasmfiend".to_string()));
        let demon_spawn = build_data.map.xy_idx(x - 10, y - 10);
        build_data
            .spawn_list
            .push((demon_spawn, "Vokoth".to_string()));
        let demon_spawn = build_data.map.xy_idx(x + 10, y - 10);
        build_data
            .spawn_list
            .push((demon_spawn, "Vokoth".to_string()));
    }
}
