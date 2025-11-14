class Matrix4x4:
    def __init__(self, data):
        self.data = data

    def __getitem__(self, index: tuple):
        (x,y) = index
        return self.data[x][y]