use std::io;

#[derive(Debug)]
struct ApartemenBooking {
    booking_id: String,
    guest_name: String,
    room_type: String,
    check_in_date: String,
    check_out_date: String,
}

fn book_apartemen(daftar_booking: &mut Vec<ApartemenBooking>) {
    println!("Booking Apartemen");

    println!("Masukkan ID Booking:");
    let mut booking_id = String::new();
    io::stdin().read_line(&mut booking_id).expect("Gagal membaca ID Booking");

    println!("Masukkan Nama Tamu:");
    let mut guest_name = String::new();
    io::stdin()
        .read_line(&mut guest_name)
        .expect("Gagal membaca nama tamu");

    println!("Masukkan Jenis Kamar:");
    let mut room_type = String::new();
    io::stdin()
        .read_line(&mut room_type)
        .expect("Gagal membaca jenis kamar");

    println!("Masukkan Tanggal Check-in (format: DD/MM/YYYY):");
    let mut check_in_date = String::new();
    io::stdin()
        .read_line(&mut check_in_date)
        .expect("Gagal membaca tanggal check-in");

    println!("Masukkan Tanggal Check-out (format: DD/MM/YYYY):");
    let mut check_out_date = String::new();
    io::stdin()
        .read_line(&mut check_out_date)
        .expect("Gagal membaca tanggal check-out");

    let booking= ApartemenBooking {
        booking_id: booking_id.trim().to_string(),
        guest_name: guest_name.trim().to_string(),
        room_type: room_type.trim().to_string(),
        check_in_date: check_in_date.trim().to_string(),
        check_out_date: check_out_date.trim().to_string(),
    };

    daftar_booking.push(booking);
    println!("Booking berhasil ditambahkan!")
}

fn show_bookings(daftar_booking: &Vec<ApartemenBooking>) {
    println!("Data Booking Apartemen");

    for (index, booking) in daftar_booking.iter().enumerate() {
        println!("{}. {:?}", index + 1, booking);
    }
}

fn edit_booking(daftar_booking: &mut Vec<ApartemenBooking>) {
    show_bookings(&daftar_booking);

    println!("Masukkan nomor booking yang ingin diedit:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Gagal membaca nomor booking");

    let index = match choice.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= daftar_booking.len() => num - 1,
        _ => {
            println!("Nomor booking tidak valid.");
            return;
        }
    };

    let mut new_booking: ApartemenBooking = book_apartemen_input();
    daftar_booking[index] = new_booking;

    println!("Booking berhasil diedit!");
}

fn delete_booking(daftar_booking: &mut Vec<ApartemenBooking>) {
    show_bookings(&daftar_booking);

    println!("Masukkan nomor booking yang ingin dihapus:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Gagal membaca nomor booking");

    let index = match choice.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= daftar_booking.len() => num - 1,
        _ => {
            println!("Nomor booking tidak valid.");
            return;
        }
    };

    daftar_booking.remove(index);

    println!("Booking berhasil dihapus!");
}

fn book_apartemen_input() -> ApartemenBooking {
    println!("Masukkan ID Booking:");
    let mut booking_id = String::new();
    io::stdin().read_line(&mut booking_id).expect("Gagal membaca ID Booking");

    println!("Masukkan Nama Tamu:");
    let mut guest_name = String::new();
    io::stdin()
        .read_line(&mut guest_name)
        .expect("Gagal membaca nama tamu");

    println!("Masukkan Jenis Kamar:");
    let mut room_type = String::new();
    io::stdin()
        .read_line(&mut room_type)
        .expect("Gagal membaca jenis kamar");

    println!("Masukkan Tanggal Check-in (format: DD/MM/YYYY):");
    let mut check_in_date = String::new();
    io::stdin()
        .read_line(&mut check_in_date)
        .expect("Gagal membaca tanggal check-in");

    println!("Masukkan Tanggal Check-out (format: DD/MM/YYYY):");
    let mut check_out_date = String::new();
    io::stdin()
        .read_line(&mut check_out_date)
        .expect("Gagal membaca tanggal check-out");

    ApartemenBooking {
        booking_id: booking_id.trim().to_string(),
        guest_name: guest_name.trim().to_string(),
        room_type: room_type.trim().to_string(),
        check_in_date: check_in_date.trim().to_string(),
        check_out_date: check_out_date.trim().to_string(),
    }
}

fn main() {
    let mut daftar_booking: Vec<ApartemenBooking> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Booking Apartemen");
        println!("2. Tampilkan Data Booking");
        println!("3. Edit Booking");
        println!("4. Hapus Booking");
        println!("5. Keluar");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Pilihan tidak tersedia");

        match choice.trim().parse() {
            Ok(1) => book_apartemen(&mut daftar_booking),
            Ok(2) => show_bookings(&daftar_booking),
            Ok(3) => edit_booking(&mut daftar_booking),
            Ok(4) => delete_booking(&mut daftar_booking),
            Ok(5) => {
                println!("Keluar dari program. Selamat tinggal!");
                break;
            }
            _ => println!("Pilihan tidak valid. Silahkan coba lagi."),
        }
    }
}
