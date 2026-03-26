#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

// Định nghĩa các từ khóa (Keys) để lưu trữ dữ liệu trên Blockchain
const GPA: Symbol = symbol_short!("GPA");
const TRAIN_POINT: Symbol = symbol_short!("TP");

#[contract]
pub struct UniTrustContract;

#[contractimpl]
impl UniTrustContract {
    
    /// 1. Cập nhật dữ liệu cho sinh viên (Chỉ Admin mới có quyền gọi hàm này)
    /// gpa: nhập số nguyên (ví dụ 3.97 thì nhập 397)
    /// tp: điểm rèn luyện (ví dụ 90)
    pub fn update_score(env: Env, admin: Address, student: Address, gpa: u32, tp: u32) {
        // Xác thực: Nếu không phải Admin ký giao dịch, hàm sẽ báo lỗi ngay lập tức
        admin.require_auth();
        
        // Tạo một khóa duy nhất cho mỗi sinh viên để lưu trữ
        // DataData(Address, Symbol) -> Value
        env.storage().persistent().set(&(student.clone(), GPA), &gpa);
        env.storage().persistent().set(&(student, TRAIN_POINT), &tp);
    }

    /// 2. Truy vấn dữ liệu (Hàm này công khai, ai cũng có thể xem)
    /// Trả về một Tuple: (GPA, Điểm rèn luyện)
    pub fn get_score(env: Env, student: Address) -> (u32, u32) {
        let current_gpa = env.storage().persistent()
            .get(&(student.clone(), GPA))
            .unwrap_or(0); // Nếu chưa có điểm, mặc định trả về 0
            
        let current_tp = env.storage().persistent()
            .get(&(student, TRAIN_POINT))
            .unwrap_or(0);
            
        (current_gpa, current_tp)
    }
}