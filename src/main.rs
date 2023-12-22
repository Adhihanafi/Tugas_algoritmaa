use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    let mut daftar_barang: Vec<Barang> = Vec::new();
    let mut stack_barang: Vec<Barang> = Vec::new();
    let mut queue_barang: VecDeque<Barang> = VecDeque::new();

    loop {
        println!("Pilihan:");
        println!("1. Tampilkan semua barang");
        println!("2. Tambah barang baru");
        println!("3. Edit barang");
        println!("4. Hapus barang");
        println!("5. Tambah ke Stack");
        println!("6. Tambah ke Queue");
        println!("7. Tampilkan Stack");
        println!("8. Tampilkan Queue");
        println!("9. Keluar");

        let pilihan: u32 = input("Masukkan pilihan Anda: ").trim().parse().expect("Harap masukkan angka");

        match pilihan {
            1 => tampilkan_barang(&daftar_barang),
            2 => tambah_barang(&mut daftar_barang),
            3 => edit_barang(&mut daftar_barang),
            4 => hapus_barang(&mut daftar_barang),
            5 => tambah_stack(&mut daftar_barang, &mut stack_barang),
            6 => tambah_queue(&mut daftar_barang, &mut queue_barang),
            7 => tampilkan_stack(&stack_barang),
            8 => tampilkan_queue(&queue_barang),
            9 => break,
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn input(pesan: &str) -> String {
    print!("{}", pesan);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

fn tampilkan_barang(daftar: &Vec<Barang>) {
    println!("Daftar Barang:");
    for barang in daftar {
        println!("ID: {}, Nama: {}, Jumlah: {}", barang.id, barang.nama, barang.jumlah);
    }
}

fn tambah_barang(daftar: &mut Vec<Barang>) {
    let id: u32 = input("Masukkan ID barang: ").trim().parse().expect("Harap masukkan angka");
    let nama = input("Masukkan nama barang: ");
    let jumlah: u32 = input("Masukkan jumlah barang: ").trim().parse().expect("Harap masukkan angka");

    daftar.push(Barang::baru(id, &nama, jumlah));
    println!("Barang berhasil ditambahkan!");
}

fn edit_barang(daftar: &mut Vec<Barang>) {
    let id: u32 = input("Masukkan ID barang yang akan diubah: ").trim().parse().expect("Harap masukkan angka");

    if let Some(barang) = daftar.iter_mut().find(|b| b.id == id) {
        let nama = input("Masukkan nama baru untuk barang: ");
        let jumlah: u32 = input("Masukkan jumlah baru untuk barang: ").trim().parse().expect("Harap masukkan angka");

        barang.nama = nama;
        barang.jumlah = jumlah;

        println!("Barang berhasil diubah!");
    } else {
        println!("Barang dengan ID tersebut tidak ditemukan!");
    }
}

fn hapus_barang(daftar: &mut Vec<Barang>) {
    let id: u32 = input("Masukkan ID barang yang akan dihapus: ").trim().parse().expect("Harap masukkan angka");

    if let Some(index) = daftar.iter().position(|b| b.id == id) {
        daftar.remove(index);
        println!("Barang berhasil dihapus!");
    } else {
        println!("Barang dengan ID tersebut tidak ditemukan!");
    }
}

fn tambah_stack(daftar: &mut Vec<Barang>, stack: &mut Vec<Barang>) {
    if let Some(barang) = daftar.pop() {
        stack.push(barang);
        println!("Barang berhasil ditambahkan ke Stack!");
    }
}

fn tambah_queue(daftar: &mut Vec<Barang>, queue: &mut VecDeque<Barang>) {
    if let Some(barang) = daftar.pop() {
        queue.push_back(barang);
        println!("Barang berhasil ditambahkan ke Queue!");
    }
}

fn tampilkan_stack(stack: &Vec<Barang>) {
    println!("Isi Stack:");
    for barang in stack.iter().rev() {
        println!("ID: {}, Nama: {}, Jumlah: {}", barang.id, barang.nama, barang.jumlah);
    }
}

fn tampilkan_queue(queue: &VecDeque<Barang>) {
    println!("Isi Queue:");
    for barang in queue.iter() {
        println!("ID: {}, Nama: {}, Jumlah: {}", barang.id, barang.nama, barang.jumlah);
    }
}

#[derive(Clone)]
struct Barang {
    id: u32,
    nama: String,
    jumlah: u32,
}

impl Barang {
    fn baru(id: u32, nama: &str, jumlah: u32) -> Self {
        Self {
            id,
            nama: nama.to_string(),
            jumlah,
        }
    }
}
