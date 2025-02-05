#pragma once

#include <iostream>
#include <string>
#include <cmath>

void quadratic_equation(){
	float a, b, c, d, x1, x2, i, realPart;
 std::cout << "Enter a, b, and c:" << std::endl ;
 std::cin >> a >> b >> c;
 d = b*b - 4*a*c;
 realPart = -b / (2*a);
 if(d > 0){
 x1 = (-b + sqrt(d)) / (2*a);
 x2 = (-b - sqrt(d)) / (2*a);
 std::cout << "Roots are real and different" <<std::endl << "x1 : " << x1 <<" " << " x2: "<< x2 <<std::endl;
 
 }
 else if(d == 0){
 // I'm to lazy to write x1 again, but it will be correct anyways so bye
 std::cout << "Roots are real and same :" << realPart << std::endl;

 }
 else {
 i = sqrt(-d) / (a*2);
 std::cout << "Roots are complex and different";
 std::cout << std::endl << "x1 " << realPart << "+" << i << "i";
 std::cout << std::endl << "x2 " << realPart << "-" << i << "i";

 }
	
}

