fn f_to_c(temp_f: f32) -> f32 {
	// TODO ergänzen Sie hier Ihre Lösung von Teilaufgabe a.
	return (temp_f-32.0)*5.0/9.0;
}

fn ggt(mut zahl_1: i32, mut zahl_2: i32) -> i32 {
	// TODO ergänzen Sie hier Ihre Lösung von Teilaufgabe b.
	if zahl_1==0 {
		return zahl_2;
	}
	while zahl_2!=0 {
		if zahl_1>zahl_2 {
			zahl_1 = zahl_1-zahl_2;
		}
		else {
			zahl_2 = zahl_2-zahl_1;
		}
	}
	return zahl_1;
}

fn buchstabe_in_zeichenkette(zeichenkette: &str, c: char) -> i32 {
	// TODO ergänzen Sie hier Ihre Lösung von Teilaufgabe c.
	let mut count:i32 = 0;
	for s in zeichenkette.chars() {
		if s==c {
			count+=1;
		}
	}
	return count;
}

fn main() {

	let temp_f: f32 = 50.0;
	println!("{} Grad Fahrenheit sind {} Grad Celsius.",temp_f, f_to_c(temp_f));

	let zahl_1: i32 = 250;
	let zahl_2: i32 = 120;
	println!("Der ggT von {} und {} ist {}.", zahl_1, zahl_2, ggt(zahl_1, zahl_2));

	let s: &str = "Universität Duisburg-Essen";
	let c = 's';

	println!("In der Zeichenkette \"{}\" kommt der Buchstabe '{}' genau {} mal vor.", s, c, buchstabe_in_zeichenkette(s, c));
}