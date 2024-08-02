#include <iostream>
#include <Eigen/Dense>

using namespace std;
using namespace Eigen;
int main(int argc, char** argv)
{
    cout << "Hello World \n";

    Matrix2d mat;
    mat(0, 0) = 3;
    mat(1, 0) = 2.5;
    mat(0, 1) = -1;
    mat(1, 1) = mat(1, 0) + mat(0, 1);
    cout << mat << endl;
    return 0;
}