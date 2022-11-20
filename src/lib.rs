pub fn square_series_print(start:i32,end:i32)
{
    for index in start..end
    {
        print!("{}  ", index * index);
    }
}