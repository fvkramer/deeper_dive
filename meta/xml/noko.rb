require 'nokogiri'
require 'byebug'

doc = File.open("test.xml") { |f| Nokogiri::XML(f) } 
xml_doc  = Nokogiri::XML("<root><aliens><alien><name>Alf</name></alien></aliens></root>")

doc.css('name').each do |name|
  name.content += ' Modified'
end

puts doc.at_css 'name'
