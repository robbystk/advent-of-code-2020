require 'set'

def input
  ARGV[0]
end

numbers = input.split(',').map(&:to_i)
previously_said = Set.new(numbers[..-2])

(2020 - numbers.length).times do
  number_to_say = if previously_said.include? numbers.last
                    last_said = numbers[0..-2].rindex numbers.last
                    numbers.length - 1 - last_said
                  else
                    0
                  end
    previously_said.add numbers.last
    numbers << number_to_say
end

puts numbers.last
