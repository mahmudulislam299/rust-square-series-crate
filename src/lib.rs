pub mod cube;


/// print square series
///
/// # Examples
///
/// ```
/// square_series_print(1,10);
/// ```

pub fn square_series_print(start:i32,end:i32)
{
    for index in start..=end
    {
        print!("{}  ", index * index);
    }
}