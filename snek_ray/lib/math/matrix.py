class Matrix:
    def __init__(self, data):
        self.data = data

    def __getitem__(self, index: tuple):
        (row,col) = index
        return self.data[row][col]
    
    def __eq__(self, value):
        return self.data == value.data