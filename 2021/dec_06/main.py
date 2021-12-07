import os
import pathlib

def readfile(file):
  return [int(x) for x in pathlib.Path(os.path.join(pathlib.Path(__file__).parent.resolve(), "inputs", file)).read_text().split(",")]

def readinput():
  return readfile("input.in")

def create_lifetime_list(data):
  life_times = [0] * 9
  for d in data:
    life_times[d] += 1

  return life_times

def calculate_nr_fish(data, nr_days):
  life_times = create_lifetime_list(data)

  day = 0
  while day < nr_days:
    new_lanternfish = life_times[0]
    for time in range(0, len(life_times)-1):  
      life_times[time] = life_times[time+1]
    
    life_times[6] += new_lanternfish
    life_times[8] = new_lanternfish
    
    day +=1
    #print (f"{life_times} ({sum(life_times)})")

  return sum(life_times)

if __name__ == '__main__':
  data = readinput()
  print(f"Solution 1: {calculate_nr_fish(data, 80)}")
  print(f"Solution 2: {calculate_nr_fish(data, 256)}")