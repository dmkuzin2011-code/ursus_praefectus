-- Temperature guard robot for Ursus
-- Reacts to temperature readings and can trigger actions

local M = {}

-- Configuration thresholds
local DEFAULT_THRESHOLD = 65

function M.tick(ctx)
    -- Read temperature from ctx parameters
    local temperature = ctx.temperature or 0
    local threshold = ctx.threshold or DEFAULT_THRESHOLD

    local report = {
        status = "ok",
        action = "normal",
        target = "none",
        message = "",
        metrics = {
            temperature = temperature,
            threshold = threshold,
        },
        timestamp = os.time(),
    }

    -- Check if temperature exceeds threshold
    if temperature > threshold then
        report.action = "estop"
        report.target = "cooling"
        report.message = string.format(
            "Temperature %.1fC exceeds threshold %.1fC",
            temperature, threshold
        )
    elseif temperature > threshold * 0.9 then
        report.action = "slow_down"
        report.target = "fan"
        report.message = string.format(
            "Temperature %.1fC approaching threshold %.1fC",
            temperature, threshold
        )
    end

    return report
end

-- Action: check temperature threshold
function M.check(params)
    local threshold = params.threshold or DEFAULT_THRESHOLD
    local current_temp = params.temperature or 0

    if current_temp > threshold then
        return {
            action = "estop",
            message = string.format(
                "Temperature %.1fC exceeds critical threshold %.1fC",
                current_temp, threshold
            ),
        }
    end

    return {
        action = "ok",
        message = string.format(
            "Temperature %.1fC is within safe threshold %.1fC",
            current_temp, threshold
        ),
    }
end

-- Action: get current status
function M.status(params)
    local temperature = params.temperature or 0
    return {
        action = "ok",
        temperature = temperature,
        message = "Robot status ok",
    }
end

return M