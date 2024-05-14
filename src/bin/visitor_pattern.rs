trait Pastry<R> {
    fn accept<T: PastryVisitor<R>>(&mut self, visitor: &mut T);
}

trait PastryVisitor<R> {
    fn visit_beignet(&mut self, beignet: &mut Beignet) -> R;
    fn visit_cruller(&mut self, cruller: &Cruller) -> R;
}

struct Beignet {
    name: String,
}

struct Cruller {
    name: String,
}

impl Beignet {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Cruller {
    fn get_name(&self) -> &String {
        &self.name
    }
}

struct Bake;

impl<R> Pastry<R> for Beignet {
    fn accept<T: PastryVisitor<R>>(&mut self, visitor: &mut T) {
        visitor.visit_beignet(self);
    }
}

impl<R> Pastry<R> for Cruller {
    fn accept<T: PastryVisitor<R>>(&mut self, visitor: &mut T) {
        visitor.visit_cruller(self);
    }
}

impl PastryVisitor<String> for Bake {
    fn visit_beignet(&mut self, beignet: &mut Beignet) -> String {
        println!("Baking Beignet with name: {}", beignet.get_name());

        "test".to_string()
    }

    fn visit_cruller(&mut self, cruller: &Cruller) -> String {
        println!("Baking Cruller with name: {}", cruller.get_name());

        "test".to_string()
    }
}

fn main() {
    let mut beignet: Beignet = Beignet {
        name: "test_beignet".to_string(),
    };

    let mut cruller: Cruller = Cruller {
        name: "test_beignet".to_string(),
    };

    let mut baker: Bake = Bake {};

    beignet.accept(&mut baker);
    cruller.accept(&mut baker);
}
