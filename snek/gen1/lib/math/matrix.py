class Matrix:
    def __init__(self, data):
        self.height = len(data)
        self.width = len(data[0]) if self.height > 0 else 0
        self.data = data

    def __getitem__(self, index: tuple):
        (row,col) = index
        return self.data[row][col]
    
    def __eq__(self, value):
        return self.data == value.data
    
    def __repr__(self):
        lines = []
        for row in self.data:
            lines.append(str(row))
        
        return "\n".join(lines)
    def __mul__(self, other):
        if self.width != other.height:
            raise ValueError("Incompatible matrix sizes for multiplication")
        
        # Limit to 4x4 for now
        if self.height > 4 or self.width > 4 or other.width > 4:
            raise ValueError("Matrix multiplication only supported for 4x4 matrices")

        result_data = []
        
        for row in range(self.height):
            result_row = []
            A = self
            B = other
            for col in range(other.width):
                result_row.append(A[(row,0)] * B[(0,col)] + 
                                  A[(row,1)] * B[(1,col)] + 
                                  A[(row,2)] * B[(2,col)] + 
                                  A[(row,3)] * B[(3,col)])
            result_data.append(result_row)

        return Matrix(result_data)