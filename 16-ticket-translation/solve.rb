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

ticket_scanning_error_rate = 0

nearby_tickets.each do |ticket|
  ticket.each do |value|
    unless rules.values.any? do |ranges|
            ranges.any? { |range| range.include? value }
          end
      ticket_scanning_error_rate += value
    end
  end
end

puts ticket_scanning_error_rate
