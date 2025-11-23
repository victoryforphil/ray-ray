def IdentityMatrix4x4():
    return Matrix([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]])


class Matrix:
    def __init__(self, data):
        self.height = len(data)
        self.width = len(data[0]) if self.height > 0 else 0
        self.data = [[float(x) for x in row] for row in data]



    def __getitem__(self, index: tuple):
        (row, col) = index
        return self.data[row][col]

    def __eq__(self, value):
        return self.data == value.data

    def __repr__(self):
        lines = []
        for row in self.data:
            lines.append(str(row))

        return "\n".join(lines)

    def __mul__(self, other):
        # Limit to 4x4 for now
        if self.height > 4 or self.width > 4:
            raise ValueError("Matrix multiplication only supported for 4x4 matrices")

        if isinstance(other, tuple):
            # Tuple multiplication
            result = []
            for row in range(self.height):
                value = (
                    self[(row, 0)] * other[0]
                    + self[(row, 1)] * other[1]
                    + self[(row, 2)] * other[2]
                    + self[(row, 3)] * other[3]
                )
                result.append([value])
            return Matrix(result)

        result_data = []

        for row in range(self.height):
            result_row = []
            A = self
            B = other
            for col in range(other.width):
                result_row.append(
                    A[(row, 0)] * B[(0, col)]
                    + A[(row, 1)] * B[(1, col)]
                    + A[(row, 2)] * B[(2, col)]
                    + A[(row, 3)] * B[(3, col)]
                )
            result_data.append(result_row)

        return Matrix(result_data)

    def transpose(self):
        result_data = []
        for col in range(self.width):
            result_row = []
            for row in range(self.height):
                result_row.append(self[(row, col)])
            result_data.append(result_row)
        return Matrix(result_data)

    def determinant(self):
        det = 0.

        if self.width == 2 and self.height == 2:
            det = self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        else:
            for col in range(self.width):
                cofactor = self.cofactor(0, col)
                det += self[(0, col)] * cofactor

        return det

    def submatrix(self, del_row, del_col):
        data = []
        for i_row in range(self.height):
            row = []
            if i_row == del_row:
                continue
            for i_col in range(self.width):
                if i_col == del_col:
                    continue
                value = self[(i_row, i_col)]
                row.append(value)

            data.append(row)
        return Matrix(data)

    def minor(self, row, col):
        mb = self.submatrix(row, col)
        return mb.determinant()

    def cofactor(self, row, col):
        minor = self.minor(row, col)
        # If row+col is ODD, negate
        negate = (row + col) % 2 != 0

        if negate:
            minor = minor * -1.0

        return minor
