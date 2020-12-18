require 'set'

input = File.open(ARGV[0]) do |f|
  f.readlines.map(&:strip)
end

mode = :rules
rules = {}
my_ticket = []
nearby_tickets = []
input.each do |line|
  case line
  when "your ticket:"
    mode = :your_ticket
  when "nearby tickets:"
    mode = :nearby_tickets
  when ""
    next
  else
    case mode
    when :rules
      field, _, ranges = line.partition(': ')
      ranges = ranges.split(' or ')
      rules[field.to_sym] = ranges.map do |range|
        low, _, high = range.partition('-')
        Range.new(low.to_i, high.to_i)
      end
    when :your_ticket
      my_ticket = line.split(',').map(&:to_i)
    when :nearby_tickets
      nearby_tickets << line.split(',').map(&:to_i)
    end
  end
end

valid_tickets = nearby_tickets.filter do |ticket|
  ticket.all? do |value|
    rules.values.any? do |ranges|
      ranges.any? { |range| range.include? value }
    end
  end
end

field_count = rules.length
field_position_candidates = field_count.times.map { Set.new(rules.keys) }
determined_indices = Set.new()

ticket_index = 0
while determined_indices.length < field_count
  # check for indexes whose field has been determined
  field_position_candidates.each_with_index do |candidates, index|
    unless determined_indices.include? index
      if candidates.length == 1
        determined_indices.add(index)
      end
    end
  end

  # eliminate fields that have been accounted for from other indexes
  determined_indices.each do |determined_index|
    field = field_position_candidates[determined_index].to_a.first
    field_position_candidates.each_with_index do |candidates, index|
      next if index == determined_index
      candidates.delete(field)
    end
  end

  # check the next ticket
  ticket = valid_tickets[ticket_index]
  field_position_candidates.each.zip(ticket) do |candidates, value|
    rules.each do |field, ranges|
      unless ranges.any? { |range| range.include? value }
        candidates.delete(field)
      end
    end
  end
  ticket_index += 1
  if ticket_index >= valid_tickets.length
    ticket_index = 0
  end

  # puts field_position_candidates.inspect, determined_indices.inspect
end

field_positions = field_position_candidates.map { |candidates| candidates.to_a.first }

field_index_map = field_positions.each_with_index.to_h


product = 1
field_index_map.each do |field, index|
  if field.to_s.start_with? 'departure'
    product *= my_ticket[index]
  end
end

puts product
