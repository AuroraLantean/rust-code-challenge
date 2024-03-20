/*
The path starts with a single slash '/'.
Any two directories are separated by a single slash '/'.

The path does not end with a trailing '/'.

The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period '.' or double period '..')
*/
struct Solution;

impl Solution {
    fn simplify_path(path: String) -> String {
        println!("--------== 71 simplify_path. path:{}", path);
        let mut stack: Vec<&str> = vec![];
        let mut res = "".to_string();
        for s in path.split_terminator('/') {
            println!("new loop s:{}, stack: {:?}", s, stack);
            match s {
                ".." => {
                    stack.pop();
                    //println!("stack.pop(): {:?}", stack);
                }
                "" | "." => {
                    continue;
                }
                _ => {
                    stack.push(s);
                }
            }
        }
        for s in stack {
            res += "/";
            res += s;
        }
        if res.is_empty() {
            res += "/";
        }
        println!("res: {:?}", res);
        res
    }
}

#[test]
fn test71() {
    let path = "/home/".to_string();
    let res = "/home".to_string();
    assert_eq!(Solution::simplify_path(path), res);

    let path = "/../".to_string();
    let res = "/".to_string();
    assert_eq!(Solution::simplify_path(path), res);

    let path = "/home//foo/".to_string();
    let res = "/home/foo".to_string();
    assert_eq!(Solution::simplify_path(path), res);

    let path = "/a/./b/../../c/".to_string();
    let res = "/c".to_string();
    assert_eq!(Solution::simplify_path(path), res);

    let path = "/a/../../b/../c//.//".to_string();
    let res = "/c".to_string();
    assert_eq!(Solution::simplify_path(path), res);

    let path = "/a//b////c/d//././/..".to_string();
    let res = "/a/b/c".to_string();
    assert_eq!(Solution::simplify_path(path), res);
}
