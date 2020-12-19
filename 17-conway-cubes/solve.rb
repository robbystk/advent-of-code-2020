require 'set'

def input
  File.open(ARGV[0]) do |f|
    f.readlines
  end
end

def augment_range(range)
  Range.new(range.begin - 1, range.end + 1)
end

class Grid
  attr_reader :grid, :x_range, :y_range, :z_range, :w_range

  def initialize(grid: Set.new)
    @grid = grid
    update_ranges
  end

  def update_ranges
    limit_array = [[nil, nil], [nil, nil], [nil, nil], [nil, nil]]
    grid.each do |point|
      point.each_with_index do |coord, i|
        # update minimum
        if !limit_array[i][0] || limit_array[i][0] > coord
          limit_array[i][0] = coord
        end
        # update maximum
        if !limit_array[i][1] || limit_array[i][1] < coord
          limit_array[i][1] = coord
        end
      end
    end

    @x_range = Range.new(limit_array[0][0], limit_array[0][1])
    @y_range = Range.new(limit_array[1][0], limit_array[1][1])
    @z_range = Range.new(limit_array[2][0], limit_array[2][1])
    @w_range = Range.new(limit_array[3][0], limit_array[3][1])
  end

  def active?(x,y,z,w)
    grid.include?([x,y,z,w])
  end

  def inactive(x,y,z,w)
    !active?(x,y,z,w)
  end

  def activate_cell(x,y,z,w)
    grid.add([x,y,z,w])
  end

  def inactivate_cell(x,y,z,w)
    grid.delete([x,y,z,w])
  end

  SEARCH_RANGE = -1..1
  def neighbor_count(x,y,z,w)
    count = 0
    SEARCH_RANGE.each do |dz|
      SEARCH_RANGE.each do |dy|
        SEARCH_RANGE.each do |dx|
          SEARCH_RANGE.each do |dw|
            if active?(x+dx, y+dy, z+dz, w+dw)
              count += 1
            end
          end
        end
      end
    end
    count
  end

  STAY_ALIVE_RANGE = 3..4
  COME_ALIVE_RANGE = 3..3

  def step
    old_grid = Grid.new(grid: grid.dup)

    augment_range(z_range).each do |z|
      augment_range(y_range).each do |y|
        augment_range(x_range).each do |x|
          augment_range(w_range).each do |w|
            neighbor_count = old_grid.neighbor_count(x,y,z,w)
            if active?(x,y,z,w)
              unless STAY_ALIVE_RANGE.include? neighbor_count
                inactivate_cell(x,y,z,w)
              end
            else
              if COME_ALIVE_RANGE.include? neighbor_count
                activate_cell(x,y,z,w)
              end
            end
          end
        end
      end
    end
    update_ranges
  end

  def active_count
    grid.length
  end

  def to_s
    w_range.map do |w|
      z_range.map do |z|
        "z = #{z}, w = #{w}\n" +
        y_range.map do |y|
          x_range.map do |x|
            if grid.include?([x,y,z,w])
              '#'
            else
              '.'
            end
          end.join('')
        end.join("\n")
      end.join("\n\n")
    end.join("\n\n")
  end
end

grid = Grid.new
input.map(&:strip).each_with_index do |line, row|
  line.split('').each_with_index do |c, column|
    case c
    when '.'
    when '#'
      grid.activate_cell(column, row, 0, 0)
    else
      raise "invalid input character `#{c.inspect}`"
    end
  end
end
grid.update_ranges

puts grid

6.times do |i|
  puts "cycle #{i}"
  grid.step
end

puts grid.active_count
