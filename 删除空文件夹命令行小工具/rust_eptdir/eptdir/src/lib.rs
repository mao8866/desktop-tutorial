// std::fs - 文件系统操作（读取目录、删除文件等）
// std::path - 路径处理（Path, PathBuf 等）
use std::fs;
use std::path::Path;

// 垃圾文件列表常量
pub const JUNK_FILES: &[&str] = &["thumbs.db", ".DS_Store"];

// 函数返回类型说明：
// Result<(), Box<dyn std::error::Error>>
//   - Result: Rust 的错误处理类型，表示可能成功或失败
//   - (): 单元类型，表示成功时不需要返回值（只有成功/失败的状态）
//   - Box<dyn std::error::Error>: 失败时盒子里是任意类型的错误对象
//     * Box: 堆分配的智能指针，用于存储不同大小的错误对象
pub fn clean_directory(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    //检查目录是否存在
    if !target_dir.exists() {
        // eprintln! 是错误输出宏
        eprintln!("警告: 目录不存在，跳过: {}", target_dir.display());
        return Ok(()); // 返回成功，但跳过这个目录
    }
    
    // 检查是否是目录
    if !target_dir.is_dir() {
        eprintln!("警告: 该路径不是目录，跳过: {}", target_dir.display());
        return Ok(()); // 返回成功，但跳过这个路径
    }
    
    println!("开始清理目录: {}", target_dir.display());
    println!(); // 空行
    
    //删除垃圾文件
    println!("删除垃圾文件...");
    
    // match 表达式：模式匹配 Result 类型
    // remove_junk_files() 返回 Result<usize, Error>
    match remove_junk_files(target_dir) {
        // Ok 分支：成功删除垃圾文件
        Ok(count) => {
            // count 是删除的文件数量
            println!("已删除 {} 个垃圾文件", count);
        }
        // Err 分支：删除过程中出错
        Err(e) => {
            // e 是错误对象，自动转换为字符串
            eprintln!("删除垃圾文件时出错: {}", e);
            return Err(e);
        }
    }
    
    println!(); // 空行

    println!("删除空文件夹...");
    
    // match 表达式：模式匹配 Result 类型
    match remove_empty_dirs(target_dir) {
        // Ok 分支：成功（不关心返回值，用 _ 忽略）
        Ok(_) => {
            println!("空文件夹清理完成");
        }
        // Err 分支：出错
        Err(e) => {
            eprintln!("删除空文件夹时出错: {}", e);
            return Err(e); // 返回错误
        }
    }
    
    println!(); // 空行
    println!("目录清理完成: {}", target_dir.display());
    println!(); // 空行
    
    Ok(()) // 返回成功
}


pub fn remove_junk_files(dir: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    // mut 关键字表示这个变量可以修改（可变变量）
    // usize 是 Rust 中的无符号整数类型，用于计数
    let mut deleted_count = 0;

    // fs::read_dir(dir) 读取目录内容返回 Result<ReadDir, Error>
    // ? 操作符：如果出错就立即返回错误，如果成功就继续执行
    let entries = fs::read_dir(dir)?;
    

    // 第一阶段：先删除垃圾文件（深度优先，从外到内）
    // entries 是一个迭代器，可以逐个访问目录中的文件/文件夹
    for entry in entries {
        // entry 的类型是 Result<DirEntry
        // 使用 ? 操作符：如果 entry 是 Err，就返回错误；如果是 Ok，则取出 DirEntry
        let entry = entry?;
        
        // entry.path() 获取这个条目的完整路径
        // path 的类型是 PathBuf（可变的路径类型）
        let path = entry.path();
        
        //检查路径是否是文件，只处理文件，跳过目录
        if path.is_file() {

            // 获取文件名
            // path.file_name() 返回 Option<&OsStr>
            // Option 表示可能没有值（None）或有值（Some(值)）
            // if let Some(file_name) = ... 如果是 Some，将值绑定到 file_name；如果是 None，跳过
            // 这里处理路径可能没有文件名的情况
            if let Some(file_name) = path.file_name() {

                // file_name 是 &OsStr 类型，需要转换为字符串
                // to_str() 返回 Option<&str>，因为文件名可能包含无效的 UTF-8
                if let Some(name_str) = file_name.to_str() {

                    // name_str 现在是 &str 类型（字符串切片）
                    // JUNK_FILES.contains(&name_str) 检查文件名是否在垃圾文件列表中
                    // &name_str 创建一个引用，因为 contains 需要引用类型
                    if JUNK_FILES.contains(&name_str) {

                        // 打印要删除的文件路径
                        // path.display() 将路径转换为可显示的字符串
                        println!("删除垃圾文件: {}", path.display());
                        
                        // 删除文件
                        // fs::remove_file() 删除文件，返回 Result
                        // ? 操作符处理可能的错误（比如文件被占用等）
                        fs::remove_file(&path)?;
                        
                        // 增加删除计数
                        deleted_count += 1;
                    }
                }
            }
        }
        //如果是目录，递归处理
        else if path.is_dir() {
            // 递归调用：自己调用自己
            // 这就是递归函数的核心：函数调用自己来处理子目录
            // &path 传递路径的引用（注意这里的生命周期）
            // ? 操作符处理可能的错误
            // 累加子目录中删除的文件数
            deleted_count += remove_junk_files(&path)?;
        }
    }
    
    // Ok() 表示成功，返回删除的文件数量
    Ok(deleted_count)
}


pub fn remove_empty_dirs(dir: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    // 检查路径是否存在
    // dir.exists() 检查路径是否存在
    if !dir.exists() {
        // Ok(false) 表示目录不存在，返回 false（未删除）
        return Ok(false);
    }
    
    // 检查是否是目录
    if !dir.is_dir() {
        // 如果是文件而不是目录，返回 false
        return Ok(false);
    }
    
    // 先递归处理所有子目录
    // if let 模式匹配：如果读取目录成功，就进入这个分支
    // Ok(entries) 表示成功读取目录
    if let Ok(entries) = fs::read_dir(dir) {
        // Vec<PathBuf> 是一个可增长的数组（动态数组）
        // mut 表示可变，可以添加元素
        // PathBuf 是路径的可变类型（String 是字符串的可变类型，PathBuf 是路径的可变类型）
        let mut subdirs: Vec<std::path::PathBuf> = Vec::new();
        
        // 收集所有子目录路径
        // 为什么先收集？因为如果我们在遍历时删除目录，迭代器可能会失效
        // 所以先收集所有子目录路径，然后再处理
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // 只收集子目录
            if path.is_dir() {
                // push() 方法将元素添加到数组末尾
                subdirs.push(path);
            }
            // 注意：如果存在文件，我们暂时不处理
            // 等递归删除所有子目录后，再检查当前目录是否为空
            // 如果目录中有文件，那目录肯定不为空，不需要删除
        }
        
        // 递归处理每个子目录
        // for 循环遍历所有收集到的子目录
        for subdir in subdirs {
            // 递归调用：自己调用自己处理子目录
            // let _ = 表示我们不关心返回值（用 _ 忽略）
            // ? 操作符处理错误
            let _ = remove_empty_dirs(&subdir)?;
        }
    }
    
    // 递归删除子目录后，再次检查当前目录是否为空
    // 为什么再次检查？ 因为删除子目录后，当前目录可能也变成空的了
    // match 表达式：模式匹配，类似于 switch 语句，但更强大
    match fs::read_dir(dir) {
        // Ok 分支：成功读取目录
        Ok(mut entries) => {
            // mut entries 表示 entries 是可变的（因为我们要调用 next()）
            // entries.next() 获取迭代器的下一个元素
            // is_none() 检查是否是 None（没有更多元素）
            // 如果目录为空，迭代器就没有任何元素
            if entries.next().is_none() {
                // 目录为空，可以删除
                println!("删除空文件夹: {}", dir.display());
                
                // 删除目录
                fs::remove_dir(dir)?;
                
                // 返回 true 表示目录被删除
                return Ok(true);
            }
            // 如果目录不为空（有文件或子目录），继续执行后面的代码
        }
        // Err 分支：读取目录失败
        Err(_) => {
            // _ 表示我们不关心具体的错误类型
            // 如果读取失败，可能目录已经被删除了（被其他进程删除等）
            // 返回 true 表示目录已经不存在了
            return Ok(true);
        }
    }
    
    // 如果执行到这里，说明目录不为空，没有被删除
    Ok(false)
}

// 声明测试模块
// 只在运行测试时编译这个模块
#[cfg(test)]
mod test;
