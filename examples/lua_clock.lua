return {
    -- Configuration for this component
    config = {
        update_interval = 1000, -- milliseconds
        show_seconds = true
    },

    -- Update function (optional)
    update = function()
        -- This could update internal state or fetch external data
    end,

    -- Render function (required)
    render = function(colorize)
        local show_seconds = true -- default fallback
        local time = os.date("%H:%M")
        if show_seconds then
            time = os.date("%H:%M:%S")
        end

        if colorize then
            local hour = tonumber(os.date("%H"))
            local color = "yellow"
            if hour < 6 or hour >= 18 then
                color = "magenta"
            end
            return {time, color}
        else
            return {time, nil}
        end
    end
}