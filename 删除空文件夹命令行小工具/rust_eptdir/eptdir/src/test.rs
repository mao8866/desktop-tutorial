// ========================================
// 单元测试模块
// ========================================
// #[cfg(test)] 表示这个模块只在运行测试时编译
// 这样测试代码不会包含在最终的程序中
#[cfg(test)]
//创建一个测试模块，把所有测试组织在一起
mod tests {
    // 导入 crate 根模块的所有内容，这样测试中可以使用所有函数
    // 使用 crate::* 而不是 super::*，因为 test 模块是 lib.rs 的子模块
    use crate::*;
    // 导入文件系统操作（创建、删除文件等）
    use std::fs;
    //导入路径类型
    use std::path::PathBuf;
    use std::io::Write;
    
    // ========================================
    // 辅助函数：创建临时测试目录
    // ========================================
    // 这个函数用于在测试中创建一个空的文件夹（目录），测试结束后可以删除
    fn create_test_dir() -> PathBuf {
        // 使用系统临时目录
        let mut test_dir = std::env::temp_dir();
        // 添加一个唯一的目录名（使用时间戳计算自 UNIX 纪元（1970-01-01）的纳秒数，作为唯一标识）
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        // 用来获取当前正在运行的线程的唯一标识符（ID），使用线程ID进一步确保唯一性
        let thread_id = std::thread::current().id();
        test_dir.push(format!("eptdir_test_{}_{:?}", timestamp, thread_id));
        // 如果目录已存在，尝试删除后重新创建
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).ok();
        }
        // 创建目录
        fs::create_dir_all(&test_dir).unwrap();
        test_dir
    }
    
    // ========================================
    // 测试 remove_junk_files 函数
    // ========================================
    
    // #[test] 属性标记这是一个测试函数
    // 测试函数名通常以 test_ 开头，描述要测试的内容
    // 测试删除 thumbs.db 垃圾文件
    #[test]
    fn test_remove_junk_files_with_thumbs_db() {
        // 准备测试环境，生成测试目录
        let test_dir = create_test_dir();
        // 构建垃圾文件路径
        let junk_file = test_dir.join("thumbs.db");
        // 创建thumbs.db垃圾文件
        fs::File::create(&junk_file).unwrap();
        
        // 执行被测试的函数
        let result = remove_junk_files(&test_dir);
        // assert! 宏：如果条件为 false，测试失败
        assert!(result.is_ok(), "函数执行成功");
        // 验证文件确实被删除（不仅检查返回值，还检查实际文件系统状态）
        assert!(!junk_file.exists(), "thumbs.db 文件被删除");
        // 清理删除测试目录
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // 测试删除 .DS_Store 文件
    #[test]
    fn test_remove_junk_files_with_ds_store() {
        let test_dir = create_test_dir();
        let junk_file = test_dir.join(".DS_Store");
        fs::File::create(&junk_file).unwrap();
        
        // 执行被测试的函数
        let result = remove_junk_files(&test_dir);
        // 检查操作是否成功执行，并验证删除的文件数量
        // expect 会在出错时 panic，assert_eq! 会验证返回值是否正确
        assert_eq!(result.expect("删除文件失败"), 1, "删除 1 个.DS_Store 文件");
        // 验证文件确实被删除（不仅检查返回值，还检查实际文件系统状态）
        assert!(!junk_file.exists(), ".DS_Store文件被删除");
        
        // 清理删除测试目录
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // 测试不删除普通文件（普通文件不是垃圾文件）
    #[test]
    fn test_remove_junk_files_with_normal_file() {
        let test_dir = create_test_dir();
        let normal_file = test_dir.join("normal.txt");
        // 创建一个普通文件
        let mut file = fs::File::create(&normal_file).unwrap();
         // 写入测试内容
        file.write_all(b"test").unwrap();
        
        let result = remove_junk_files(&test_dir);
        // result.is_ok() 检查操作是否成功执行（没有出错）
        // 注意：Ok 只表示"执行成功"，不表示"删除了文件"
        assert!(result.is_ok());
        // Ok(0) = 成功执行 + 删除了 0 个文件
        assert_eq!(result.unwrap(), 0, "普通文件目录返回 0");
        // 普通文件还在
        assert!(normal_file.exists(), "普通文件保留");
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // 测试递归删除子目录中的垃圾文件
    #[test]
    fn test_remove_junk_files_recursive() {
        let test_dir = create_test_dir();
        let subdir = test_dir.join("subdir");
        // 确保子目录是新的（如果存在则先删除，然后创建）
        // 由于 create_test_dir() 创建的是唯一目录，理论上 subdir 不应该存在
        // 但为了防御性编程，先尝试删除再创建
        fs::remove_dir_all(&subdir).ok(); // 如果不存在，删除操作会失败但被忽略
        // 创建子文件夹
        fs::create_dir(&subdir).unwrap();
        
        // 在子目录中创建垃圾文件
        let junk_file = subdir.join("thumbs.db");
        fs::File::create(&junk_file).unwrap();
        
        // 在主目录中也创建一个垃圾文件
        let junk_file2 = test_dir.join(".DS_Store");
        fs::File::create(&junk_file2).unwrap();
        
        let result = remove_junk_files(&test_dir);
        assert!(result.is_ok());
        // 应该递归删除 2 个垃圾文件（1个在主目录，1个在子目录）
        //assert_eq! 的作用
        // 如果 result.unwrap() == 2：断言通过，测试继续
        // 如果不等于 2：断言失败，测试失败并显示期望值与实际值
        assert_eq!(result.unwrap(), 2, "递归删除返回 2");
        
        // 验证文件确实被删除（不仅检查返回值，还检查实际文件系统状态）
        assert!(!junk_file.exists(), "子目录中的垃圾文件被删除");
        assert!(!junk_file2.exists(), "主目录中的垃圾文件被删除");
        
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // 测试函数会不会因空目录而崩溃
    #[test]
    fn test_remove_junk_files_empty_dir() {
        // 测试空目录应该返回 0
        let test_dir = create_test_dir();
        
        let result = remove_junk_files(&test_dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0, "空目录返回 0");
        
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // ========================================
    // 测试 remove_empty_dirs 函数
    // ========================================
    // 只删除空目录，不删除包含文件的目录
    #[test]
    fn test_remove_empty_dirs_single_empty() {
        // 测试删除单个空目录
        let test_dir = create_test_dir();
        // 在主目录中创建一个文件，确保主目录不会被删除
        let marker_file = test_dir.join(".test_marker");
        fs::File::create(&marker_file).unwrap();
        // 创建空文件夹
        let empty_subdir = test_dir.join("empty_subdir");
        fs::create_dir(&empty_subdir).unwrap();
        
        let result = remove_empty_dirs(&test_dir);
        assert!(result.is_ok());
        // 空子目录应该被删除
        assert!(!empty_subdir.exists(), "空子目录被删除");
        // 主目录还在（因为有标记文件）
        assert!(test_dir.exists(), "主目录成功保留");
        
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // 测试删除嵌套的空目录
    #[test]
    fn test_remove_empty_dirs_nested_empty() {
        // 测试：删除嵌套的空目录
        let test_dir = create_test_dir();
        // 在主目录中创建一个文件，确保主目录不会被删除
        let marker_file = test_dir.join(".test_marker");
        fs::File::create(&marker_file).unwrap();
        
        let subdir1 = test_dir.join("subdir1");
        let subdir2 = subdir1.join("subdir2");
        let subdir3 = subdir2.join("subdir3");
        
        fs::create_dir_all(&subdir3).unwrap();
        
        let result = remove_empty_dirs(&test_dir);
        assert!(result.is_ok());
        // 所有嵌套的空目录都应该被删除
        assert!(!subdir3.exists());
        assert!(!subdir2.exists());
        assert!(!subdir1.exists());
        // 主目录应该还在（因为有标记文件）
        assert!(test_dir.exists(), "主目录保留");
        
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // 测试有文件的目录不应该被删除
    #[test]
    fn test_remove_empty_dirs_with_file() {
        let test_dir = create_test_dir();
        // 在主目录中创建一个文件，确保主目录不会被删除
        let marker_file = test_dir.join(".test_marker");
        fs::File::create(&marker_file).unwrap();
        
        let subdir = test_dir.join("subdir");
        fs::create_dir(&subdir).unwrap();
        
        // 在子目录中创建一个文件
        let file = subdir.join("file.txt");
        fs::File::create(&file).unwrap();
        
        let result = remove_empty_dirs(&test_dir);
        assert!(result.is_ok());
        // 因为有文件，子目录不应该被删除
        assert!(subdir.exists(), "有文件的目录保留");
        
        fs::remove_dir_all(&test_dir).ok();
    }
    
    // 测试不存在的目录应该返回 false
    #[test]
    fn test_remove_empty_dirs_nonexistent() {
        // 创建一个路径对象，该路径在文件系统中不存在
        let nonexistent = PathBuf::from("/nonexistent/path/that/does/not/exist");
        let result = remove_empty_dirs(&nonexistent);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false, "不存在的目录返回 false");
    }
    

    // ========================================
    // 测试 clean_directory 函数
    // ========================================

    // 测试完整的清理流程
    #[test]
    fn test_clean_directory_complete() {
        // 测试完整的清理流程
        let test_dir = create_test_dir();
        
        // 创建一些测试文件
        let junk_file = test_dir.join("thumbs.db");
        fs::File::create(&junk_file).unwrap();
        
        let normal_file = test_dir.join("normal.txt");
        fs::File::create(&normal_file).unwrap();
        
        // 创建空子目录
        let empty_subdir = test_dir.join("empty_subdir");
        fs::create_dir(&empty_subdir).unwrap();
        
        // 执行清理
        let result = clean_directory(&test_dir);
        assert!(result.is_ok(), "清理目录成功");
        
        // 验证结果
        assert!(!junk_file.exists(), "垃圾文件被删除");
        assert!(normal_file.exists(), "普通文件保留");
        assert!(!empty_subdir.exists(), "空目录被删除");
        // 主目录应该还在（因为有普通文件）
        assert!(test_dir.exists(), "主目录保留");
        
        fs::remove_dir_all(&test_dir).ok();
    }

    // 测试不存在的目录应该返回 Ok（跳过）
    #[test]
    fn test_clean_directory_nonexistent() {
        // 测试不存在的目录应该返回 Ok（跳过）
        let nonexistent = PathBuf::from("/nonexistent/path/that/does/not/exist");
        
        let result = clean_directory(&nonexistent);
        // 根据代码逻辑，不存在的目录会返回 Ok(())
        assert!(result.is_ok());
    }
    
    // 测试如果路径是文件而不是目录，应该返回 Ok（跳过）
    #[test]
    fn test_clean_directory_not_a_dir() {
        let test_dir = create_test_dir();
        let test_file = test_dir.join("test_file.txt");
        fs::File::create(&test_file).unwrap();
        
        let result = clean_directory(&test_file);
        // 根据代码逻辑，文件路径会返回 Ok(())
        assert!(result.is_ok());
        
        fs::remove_dir_all(&test_dir).ok();
    }
}

