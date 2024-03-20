/*climbing a staircase. It takes n steps to reach the top. Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 */
struct Solution;

impl Solution {
    fn climb_stairs(n: i32) -> i32 {
        println!("--------== 70 climb_stairs(fold fn). n:{}", n);
        let (mut a, mut b, mut ways) = (0, 1, 0);
        // Iterate to calculate ways using Fibonacci logic.
        #[allow(unused_variables)]
        for i in 1..=n {
            ways = a + b; // Calculate current ways.
            a = b; // Update a.
            b = ways; // Update b.
        }
        // Return total ways to climb the stairs.
        ways

        // match n {
        //     1 | 2 => n,
        //     k => (2..k).fold((1, 2), |acc, _| (acc.1, acc.0 + acc.1)).1,
        // }
    }
    pub fn giant_grunts(initial: char) -> String {
        ["Bee", "Fee", "Gee", "Fi", "Hi", "Fo", "Mo", "Fum", "Tum"]
            .iter()
            .fold(String::new(), |acc, grunt| {
                if grunt.starts_with(initial) {
                    acc + grunt
                } else {
                    acc
                }
            })
    }
}

#[test]
fn test70() {
    //let even_sum2 = (0..=10).filter(|n| *n % 2 == 0).sum()
    //let sum: u32 = vec![1, 2, 3, 4, 5, 6].iter().sum();
    #[allow(unused_variables)]
    let sum: u32 = vec![1, 2, 3, 4, 5, 6]
        .iter()
        .fold(0u32, |sum, val| sum + val);

    let even_sum = (1..=10).fold(0, |acc, num| if num % 2 == 0 { acc + num } else { acc });
    println!("even_sum: {even_sum:?}");

    let songs = Solution::giant_grunts('F');
    println!("songs: {}", songs);

    assert_eq!(Solution::climb_stairs(1), 1);

    assert_eq!(Solution::climb_stairs(2), 2);

    assert_eq!(Solution::climb_stairs(3), 3);
}
