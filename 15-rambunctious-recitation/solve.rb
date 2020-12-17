require 'set'

def input
  ARGV[0]
end

starting_numbers = input.split(',').map(&:to_i)

previously_said = {}
starting_numbers[..-2].each_with_index do |n, i|
  previously_said[n] = i + 1
end

LIMIT = 10

last_said = starting_numbers.last
turn = starting_numbers.length
while turn < LIMIT do
  number_to_say = if previously_said.include? last_said
                    turn - previously_said[last_said]
                  else
                    0
                  end
  previously_said[last_said] = turn
  last_said = number_to_say
  turn += 1
end

puts last_said
