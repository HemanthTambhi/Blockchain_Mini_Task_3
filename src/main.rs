use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io;

struct Student {
    name: String,
    total: f64,
    subjects: u32,
    average: f64,
    grade: String,
}

fn get_grade(avg: f64) -> String {
    if avg >= 90.0 {
        "A".to_string()
    } else if avg >= 75.0 {
        "B".to_string()
    } else if avg >= 60.0 {
        "C".to_string()
    } else {
        "D".to_string()
    }
}

fn main() {
    println!("How many students?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();

    let mut students: Vec<Student> = Vec::new();

    for i in 0..count {
        println!("\nStudent {} Name:", i + 1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();

        println!("Total Marks:");
        let mut total = String::new();
        io::stdin().read_line(&mut total).unwrap();

        println!("Number of Subjects:");
        let mut subs = String::new();
        io::stdin().read_line(&mut subs).unwrap();

        let name = name.trim().to_string();
        let total: f64 = total.trim().parse().unwrap();
        let subs: u32 = subs.trim().parse().unwrap();
        let avg = total / subs as f64;
        let grade = get_grade(avg);

        students.push(Student {
            name,
            total,
            subjects: subs,
            average: avg,
            grade,
        });
    }

    // Create PDF
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    let mut y = 270.0;
    layer.use_text("📄 Student Report Card", 18.0, Mm(60.0), Mm(y), &font);
    y -= 15.0;

    for student in students {
        let line = format!(
            "Name: {}, Total: {}, Subjects: {}, Average: {:.2}, Grade: {}",
            student.name, student.total, student.subjects, student.average, student.grade
        );
        layer.use_text(line, 12.0, Mm(20.0), Mm(y), &font);
        y -= 10.0;
    }

    doc.save(&mut BufWriter::new(File::create("report_card.pdf").unwrap()))
        .unwrap();

    println!("✅ PDF saved as report_card.pdf");
}