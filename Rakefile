# frozen_string_literal: true

task :generate do
  sh "tinted-builder-rust build ."
  confs = Dir.glob("./intermediate/*.conf")
  confs.each do |conf|
    content = File.read(conf)
    background = content.match(/Background=([0-9a-f]{6})/)[1]
    background = [background[0..1], background[2..3], background[4..5]].map { |c| c.to_i(16) }
    content.gsub!(/([0-9a-f]{6})@([0-9]+)%/) do |match|
      hex_color = Regexp.last_match(1)
      tint_level = Regexp.last_match(2).to_i
      r = hex_color[0..1].to_i(16)
      g = hex_color[2..3].to_i(16)
      b = hex_color[4..5].to_i(16)
      r = (r * (tint_level) + background[0] * (100 - tint_level)) / 100
      g = (g * (tint_level) + background[1] * (100 - tint_level)) / 100
      b = (b * (tint_level) + background[2] * (100 - tint_level)) / 100
      format("%02x%02x%02x", r, g, b)
    end
    File.write("./themes/#{
      File.basename(conf)
    }", content)
  end
end
