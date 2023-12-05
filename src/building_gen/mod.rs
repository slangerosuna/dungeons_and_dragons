/* 
 * @Author: Sofia Langer-Osuna
 */
use bevy::prelude::*;
use bevy_app_compute::prelude::*;

pub struct BuildingGenerator;

impl Plugin for BuildingGenerator {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AppComputePlugin)
            .add_plugin(AppComputeWorkerPlugin::<BuildingComputeWorker>::default())
            .add_system(update);
    }
}

fn update(
    buttons: Res<Input<MouseButton>>,
    mut compute_worker: ResMut<AppComputeWorker<BuildingComputeWorker>>,
) {
    if compute_worker.ready() { 
        for value in compute_worker.read_raw("values").into_iter() { 
            print!("{}", value) 
        }
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) { return; }

    compute_worker.execute();
    //compute_worker.write_slice("values", &[2., 3., 4., 5.]);

    //TODO all
}

#[derive(TypeUuid)]
#[uuid = "2545ae14-a9bc-4f03-9ea4-4eb43d1075a7"]
pub struct LineDetector;

impl ComputeShader for LineDetector {
    fn shader() -> ShaderRef {
        "shaders/lineDetector.wgsl".into()
    }
}

#[derive(Resource)]
struct BuildingComputeWorker;

impl ComputeWorker for BuildingComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let worker = AppComputeWorkerBuilder::new(world)
            //TODO add uniforms
            .add_uniform("uni", &5.)
            //TODO add staged values
            .add_staging("values", &[1., 2., 3., 4.])
            //TODO add passes
            .add_pass::<LineDetector>([4, 1, 1], &["uni", "values"])
            .one_shot()
            .build();
        worker
    }
}

pub struct building {
    //TODO all
}

/*
 * will use a model synthesis algorithm that compares the building to the reference images 
 * as a way to decide what sections to include
 *
 * creates unique sections to include for certain groups of buildings
 *
 * uses references to decide shape before filling in the details with sections
 *
 * reference: https://paulmerrell.org/model-synthesis/
 *
 * After the building is generated through the model synthesis algorithm, it will be modified
 * with something based off of gaussian splatting, but starting with the model synthesis output
 * instead of random to add details
 * 
 * References: 
 *     1. Dreamgaussian
 *         Code: https://github.com/dreamgaussian/dreamgaussian
 *         Article pdf: https://arxiv.org/pdf/2309.16653.pdf
 *     2. GraphDeco 3d Gaussian Splatting
 *         Code: https://github.com/graphdeco-inria/gaussian-splatting
 *         Article pdf: https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/3d_gaussian_splatting_high.pdf
 *     
 * Passes:
 *     1. Line Detection
 *     2. Use lines to create rough shape with model synthesis (voxel output)
 *     3. Gaussian splatting to add details
 *     4. Convert voxel output to mesh with marching cubes
 *     5. ???
 *     6. Profit
 */
