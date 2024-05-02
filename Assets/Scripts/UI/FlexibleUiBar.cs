using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Serialization;
using UnityEngine.UI;

[RequireComponent( typeof( Slider ) )]
public class FlexibleUiBar : FlexibleUi
{
    [Header( "Type Data" )]
    public BarType barType;

    public bool backgroundEnabled = true;

    [Tooltip( "Pool Asset to observe." )] public PoolSO observedPool;

    protected override void ModifySkinUi()
    {
        base.ModifySkinUi();
        if ( !skinData ) return;

        switch ( barType )
        {
            case BarType.Hp:
                var background = gameObject.transform.GetChild( 0 ).gameObject.GetComponent< Image >();
                var fill       = gameObject.transform.GetChild( 1 ).GetChild( 0 ).GetComponent< Image >();

                if ( skinData.hpBarFill is not null ) fill.sprite             = skinData.hpBarFill;
                if ( skinData.hpBarBackground is not null ) background.sprite = skinData.hpBarBackground;

                background.enabled = backgroundEnabled;
                break;
        }
    }

    public override void Update()
    {
        base.Update();
        var slider = GetComponent< Slider >();

        if ( observedPool )
        {
            slider.value = ( float )observedPool.currentValue / observedPool.baseValue;
        }
    }
}

[System.Serializable]
public enum BarType
{
    Default,
    Hp,
}