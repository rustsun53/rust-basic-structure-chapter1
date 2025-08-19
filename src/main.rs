fn main() {
    println!("Hello, Rustacean!");
	say_hello("rustsun"); // memanggil fungsi tulisan
	
    let jumlah = add(2, 3); // memanggil fungsi tambah
	println!("2 + 3 = {}", jumlah);
	let hasil_kali = multiply(2, 3); // memanggil fungsi kali
	println!("2 * 3 = {}", hasil_kali);
	let pembagian = divide(2, 3); // memanggil fungsi bagi
	println!("6 / 3 = {}", pembagian);
	let persen = percentage(20.0, 80.0); // memanggil fungsi persentase
    println!("20 dari 80 adalah {}%", persen);}
	
// DEFINISI FUNGSI DIATAS DITULIS DIBAWAH INI, JIKA TIDAK DITULIS AKAN EROR Rust butuh deklarasi yang jelas.
// Nama fungsi harus sesuai (huruf besar/kecil sensitif).
fn say_hello(name: &str) {println!("Hi, {}!", name);}

fn add(a: i32, b: i32) -> i32 {a + b}
fn multiply(a: i32, b: i32) -> i32 {a * b}
fn divide(a: i32, b: i32) -> i32 {a / b}

// f64 adalah bilangan pecahan, untuk hasil presisi hindari pembangian dengan 0.0
fn percentage(part: f64, total: f64) -> f64 { 
    if total == 0.0 {println!("⚠️ Total tidak boleh nol!");return 0.0;}
(part / total) * 100.0}

// i32 adalah tipe data integer (bilangan bulat) dalam Rust yaitu 32 bit
// i → signed (bisa negatif & positif)
// u → unsigned (hanya positif / nol)