# frozen_string_literal: true

OBJECT_COLOR_VARIANTS = {
  "original" => {
    "BASE_TEMPLATE" => "original",
    "OBJECT_VIDEO_COLOR" => "base0D-hex",
    "OBJECT_AUDIO_COLOR" => "base08-hex",
    "OBJECT_CONTROL_COLOR" => "base0C-hex",
    "OBJECT_VIDEO_FILTER_COLOR" => "base0B-hex",
    "OBJECT_AUDIO_FILTER_COLOR" => "base0A-hex",
    "OBJECT_VIDEO_EFFECT_COLOR" => "base0C-hex",
    "OBJECT_AUDIO_EFFECT_COLOR" => "base0C-hex"
  },
  "rainbow" => {
    "BASE_TEMPLATE" => "rainbow",
    "OBJECT_VIDEO_COLOR" => "base0D-hex",
    "OBJECT_AUDIO_COLOR" => "base08-hex",
    "OBJECT_CONTROL_COLOR" => "base0E-hex",
    "OBJECT_VIDEO_FILTER_COLOR" => "base0B-hex",
    "OBJECT_AUDIO_FILTER_COLOR" => "base0A-hex",
    "OBJECT_VIDEO_EFFECT_COLOR" => "base0C-hex",
    "OBJECT_AUDIO_EFFECT_COLOR" => "base09-hex"
  }
}.freeze

def render_object_color_template(content, values)
  content.gsub(/__([A-Z0-9_]+)__/) do |match|
    values.fetch(Regexp.last_match(1), match)
  end
end

task :generate_templates do
  base_template = File.read("./templates/base.mustache")
  OBJECT_COLOR_VARIANTS.each do |name, values|
    rendered = render_object_color_template(base_template, values)
    File.write("./templates/#{name}.mustache", rendered)
  end
end

task generate: :generate_templates do
  sh "tinted-builder-rust build ."
  confs = Dir.glob("./intermediate/{rainbow,original}/*.conf")
  confs.each do |conf|
    content = File.read(conf)
    background = content.match(/Background=([0-9a-f]{6})/)[1]
    background =
      [background[0..1], background[2..3], background[4..5]].map do |c|
        c.to_i(16)
      end
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
    base = conf.split("/")[-2]
    File.write("./themes/#{base}/#{File.basename(conf)}", content)
  end
end
