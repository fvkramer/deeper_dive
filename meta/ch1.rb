# class MyClass
#   def self.define_component(name)
#     define_method(name) do
#       "I am a #{name}"
#     end
#   end

#   define_component :monkey
#   define_component :meta
#   define_component :snake
# end

# test = MyClass.new

# p test.monkey
# p test.meta
# p test.snake

# #####


# def just_yield
#   yield
# end

# top_level_variable = 1

# just_yield do 
#   top_level_variable++
#   local_to_block = 1
# end

# class MyClass
#   def initialize 
#     @v = 1
#   end
# end

# obj = MyClass.new

# obj.instance_eval do 
#   p @v+2
# end

###################

# class C
#   def initialize
#     @x = 1
#   end
# end

# class D
#   def twisted_method
#     @y = 2
#     C.new.instance_eval { "@x: #{@x} ,  @y: #{@y}"}
#   end
# end

# class D
#   def twisted_method
#     @y = 2
#     C.new.instance_exec(@y) {|y| "@x:  #{@x} ,  @y:  #{y}"}
#   end
# end

# p D.new.twisted_method

#######################

def my_method(&prc)
  prc
end

p = my_method { "Hello" }
p p.class