use object_averaged_collection::AveragedCollection;

fn main() {
    let mut numbers = AveragedCollection::new();
    numbers.add(1);
    numbers.add(2);
    println!("average {}", numbers.average());
}
