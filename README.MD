# raytracer in rust

This is a toy project to learn rust, based around the following online resources

https://raytracing.github.io/books/RayTracingInOneWeekend.html

https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/


https://doc.rust-lang.org/book

https://blog.singleton.io/posts/2022-01-02-raytracing-with-rust/


http://cosinekitty.com/raytrace/



# things to do

- figure out a way to abstract the scatter/material part better. Ideas:
  - instead of the tuple of HitRecord and Material,
    should we add a function pointer to represent the scatter function?

- make the Torus class work (it looks like it is working, but it spews out way to many debug logs)

- how can this be parallelised? the "final render" is quite intensive ...
- related question: how can I debug this performance?

- use command line args to switch between the small and the big scene

