require 'set'

def input
  File.open(ARGV[0]) do |f|
    f.readlines
  end
end

class Grid
  attr_reader :grid, :x_range, :y_range, :z_range

  def initialize
    @grid = Set.new
    @x_range = 0..0
    @y_range = 0..0
    @z_range = 0..0
  end

  def add_point(x,y,z)
    @grid.add([x,y,z])
    @x_range = Range.new([x_range.begin, x].min, [x_range.end, x].max)
    @y_range = Range.new([y_range.begin, y].min, [y_range.end, y].max)
    @z_range = Range.new([z_range.begin, z].min, [z_range.end, z].max)
  end

  def to_s
    z_range.map do |z|
      "\nz = #{z}\n" +
      y_range.map do |y|
        x_range.map do |x|
          if grid.include?([x,y,z])
            '#'
          else
            '.'
          end
        end.join('')
      end.join("\n")
    end.join('')
  end
end

grid = Grid.new

input.map(&:strip).each_with_index do |line, row|
  line.split('').each_with_index do |c, column|
    case c
    when '.'
    when '#'
      grid.add_point(column, row, 0)
    else
      raise "invalid input character `#{c.inspect}`"
    end
  end
end

puts grid
