use bincode::serialize_into;
use geoprim::Plot;
use implicit_mesh::function::Function;
use implicit_mesh::interval::Interval;
use implicit_mesh::mesh_tree::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "expr_to_geom")]
struct Args {
    /// The epression to generate geometry for
    #[structopt(short = "e", long = "expression")]
    expression: String,

    /// The epsilon value that serves as the basecase for our octtree recursion
    #[structopt(short = "s", long = "epsilon")]
    epsilon: f32,

    /// The file to write out output to
    #[structopt(name = "FILE", short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// The bounding box side size
    #[structopt(short = "b", long = "bounding-box", default_value = "40.0")]
    box_size: f32,

    /// If passed, don't plot the oct tree
    #[structopt(long = "no-oct-tree")]
    no_oct_tree: bool,
}

fn main() {
    let args = Args::from_args();

    println!("Parsing...");
    let input: Vec<char> = args.expression.chars().collect();
    let f = implicit_mesh::parser::parse_expression(&input, 0).expect("Unable to parse expression");
    //let f = Box::new(implicit::function::ConstFunction{ c: 0.0});

    println!("Making mesh tree...");
    let size_interval = Interval::new(-args.box_size / 2.0, args.box_size / 2.0);
    let bounding_box = BoundingBox {
        x: size_interval.clone(),
        y: size_interval.clone(),
        z: size_interval.clone(),
    };

    let mut mtree = MeshTree::new(f, bounding_box);
    {
        println!("Plotting mtree...");
        mtree.generate_vertex_map();
        let mut plot = Plot::new();
        mtree.add_to_plot(false, false, false, true, &mut plot);
        let file = File::create(&args.output).unwrap();
        let mut w = BufWriter::new(file);
        serialize_into(&mut w, &plot).expect("Unable to serialize plot");
    }

    while mtree.level < 16 {
        let mut line = String::new();
        let input = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim() {
            "r" => {
                println!("Relaxing net...");
                mtree.relax_vertices();
            }
            _ => {
                println!("Next level...");
                mtree.next_level();
                mtree.generate_vertex_map();
                mtree.generate_triangle_set();
            }
        }

        println!("Plotting mtree...");
        let mut plot = Plot::new();
        mtree.add_to_plot(false, true, false, true, &mut plot);

        let file = File::create(&args.output).unwrap();
        let mut w = BufWriter::new(file);

        serialize_into(&mut w, &plot).expect("Unable to serialize plot");
    }
}
