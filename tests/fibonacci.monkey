let phi = fn(n) {
    if (n == 0) { return 1; }
    if (n == 1) { return 1; }
    phi(n-1) + phi(n-2);
};

puts( "Fib(", 0, ") =", phi(0) );
puts( "Fib(", 1, ") =", phi(1) );
puts( "Fib(", 2, ") =", phi(2) );
puts( "Fib(", 3, ") =", phi(3) );
puts( "Fib(", 4, ") =", phi(4) );
puts( "Fib(", 5, ") =", phi(5) );
puts( "Fib(", 6, ") =", phi(6) );
puts( "Fib(", 7, ") =", phi(7) );
puts( "Fib(", 8, ") =", phi(8) );
puts( "Fib(", 9, ") =", phi(9) );
puts( "Fib(", 10, ") =", phi(10) );
