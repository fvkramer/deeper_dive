def event(description)
  p "ALERT: #{description}" if yield
end
load 'events.rb'