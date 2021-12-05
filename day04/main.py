with open("input.txt") as f:
    lines = f.read().splitlines()
    numbers = [int(val) for val in lines[0].split(",")]
    board_vals = []
    cur_board_val = []
    for l in lines[2:]:
        if l == "":
            board_vals.append(cur_board_val)
            cur_board_val = []
            continue
        cur_board_val.append(l)
    # Don't forget to add the last one!
    board_vals.append(cur_board_val)


class Board:
    def __init__(self, vals):
        self.values = [[0] * 5 for _ in range(5)]
        self.marked = [[False] * 5 for _ in range(5)]
        self.sum = 0
        for r, row in enumerate(vals):
            for c, val in enumerate(row.split()):
                self.values[r][c] = int(val)
                self.sum += int(val)
        self.is_winner = False

    def mark(self, val):
        marked = []
        for r in range(5):
            for c in range(5):
                if self.values[r][c] != val:
                    continue
                self.marked[r][c] = True
                self.sum -= val
                marked.append((r, c))
        for r, c in marked:
            row_win = col_win = True
            for rr in range(5):
                if not self.marked[rr][c]:
                    col_win = False
                    break
            for cc in range(5):
                if not self.marked[r][cc]:
                    row_win = False
                    break
            if row_win or col_win:
                self.is_winner = True


boards = []
for vals in board_vals:
    boards.append(Board(vals))


def do():
    winners = set()
    for n in numbers:
        for i, b in enumerate(boards):
            b.mark(n)
            if b.is_winner:
                winners.add(i)
                #  print(n * b.sum)
                #  return

                # Did I hack to make sure all boards will ultimately win?
                if len(winners) == len(boards):
                    print(b.sum * n)
                    return


do()
