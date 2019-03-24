require 'thor'
require 'foodie'

module Foodie
  class CLI < Thor
    desc "portray ITEM", "Determine if a piece of food is gross or delicious"
    def portray(name)
      puts Foodie::Food.portray(name)
    end
  end
end